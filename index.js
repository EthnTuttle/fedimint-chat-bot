import { PineconeClient } from "@pinecone-database/pinecone";
import { DirectoryLoader } from "langchain/document_loaders/fs/directory"
import { TextLoader } from "langchain/document_loaders/fs/text"
import { PDFLoader } from "langchain/document_loaders/fs/pdf"
import { GithubRepoLoader } from "langchain/document_loaders/web/github"
import { PlaywrightWebBaseLoader } from "langchain/document_loaders/web/playwright"
import { FigmaFileLoader } from "langchain/document_loaders/web/figma"
import { UnstructuredLoader } from "langchain/document_loaders/fs/unstructured"
import * as dotenv from "dotenv"
import { createPineconeIndex } from "./createPinecone.js"
import { updatePinecone } from "./updatePinecone.js"
import { queryPineconeVectorStoreAndQueryLLM } from "./queryPineconeAndQueryGPT.js"


dotenv.config();

const loader = new DirectoryLoader("./data/", {
    ".txt": (path) => new TextLoader(path),
    ".pdf": (path) => new PDFLoader(path),
    ".html": (path) => new UnstructuredLoader(path)
});

const docs = await loader.load();

const githubFedimintLoader = new GithubRepoLoader("https://github.com/fedmint/fedimint", { branch: "master", unknown: "warn", recursive: false });
const githubUiLoader = new GithubRepoLoader("https://github.com/fedmint/ui", { branch: "master", unknown: "warn", recursive: false });
const fedimintOrgLoader = new GithubRepoLoader("https://github.com/fedimint/fedimint.org", { branch: "main", unknown: "warn", recursive: false });

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
