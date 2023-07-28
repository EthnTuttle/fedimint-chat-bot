import { PineconeClient } from "@pinecone-database/pinecone";
import { DirectoryLoader, UnknownHandling } from "langchain/document_loaders/fs/directory"
import { TextLoader } from "langchain/document_loaders/fs/text"
import { GithubRepoLoader } from "langchain/document_loaders/web/github"
import * as dotenv from "dotenv"
import { createPineconeIndex } from "./createPinecone.js"
import { updatePinecone } from "./updatePinecone.js"
import { queryPineconeVectorStoreAndQueryLLM } from "./queryPineconeAndQueryGPT.js"
import { UnstructuredDirectoryLoader, UnstructuredLoader } from "langchain/document_loaders/fs/unstructured";


dotenv.config();

const loader = new DirectoryLoader("./data/transcripts", {
    ".txt": (path) => new TextLoader(path),
});

const docs = await loader.load();


// RUN THIS docker run -p 8000:8000 -d --rm --name unstructured-api quay.io/unstructured-io/unstructured-api:latest --port 8000 --host 0.0.0.0
const options = {
    recursive: true,
    apiUrl: "http://localhost:8000/general/v0/general",
    unknown: 'warn',
    // apiKey: process.env.UN_API_KEY ?? ''
};

const unstructuredLoader = new UnstructuredDirectoryLoader(
    "./data/fedimint-docs",
    options,
);

docs.push(await unstructuredLoader.load());
// NOT WORKING, throws 404 from the unstructured docker container.

const githubFedimintLoader = new GithubRepoLoader(
    "https://github.com/fedmint/fedimint", 
    { accessToken: process.env.GH_PAT ?? "", branch: "master", unknown: "warn", recursive: false }
);
const githubUiLoader = new GithubRepoLoader("https://github.com/fedmint/ui", {accessToken: process.env.GH_PAT ?? "",  branch: "master", unknown: "warn", recursive: false });
const fedimintOrgLoader = new GithubRepoLoader("https://github.com/fedimint/fedimint.org", {accessToken: process.env.GH_PAT ?? "",  branch: "main", unknown: "warn", recursive: false });

docs.push(await githubFedimintLoader.load())
docs.push(await githubUiLoader.load())
docs.push(await fedimintOrgLoader.load())

const question = "What is fedimint?";
const indexName = "fedimint-index"
const vectorDimension = 1536; // needed for OpenAI embeddings, might be differect for other LLMS

const client = new PineconeClient();
await client.init({
    apiKey: process.env.PINECONE_API_KEY,
    environment: process.env.PINECONE_ENVIRONMENT,
});

(async () => {
    await createPineconeIndex(client, indexName, vectorDimension);
    await updatePinecone(client, indexName, docs);
    await queryPineconeVectorStoreAndQueryLLM(client, indexName, question);
})();
