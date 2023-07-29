import { PineconeClient } from "@pinecone-database/pinecone";
import { DirectoryLoader } from "langchain/document_loaders/fs/directory"
import { TextLoader } from "langchain/document_loaders/fs/text"
import * as dotenv from "dotenv"
import { createPineconeIndex } from "./createPinecone.js"
import { updatePinecone } from "./updatePinecone.js"
import { queryPineconeVectorStoreAndQueryLLM } from "./queryPineconeAndQueryGPT.js"


dotenv.config();

const indexName = "fedimint-index"
const vectorDimension = 1536; // needed for OpenAI embeddings, might be different for other LLMS

const client = new PineconeClient();
await client.init({
    apiKey: process.env.PINECONE_API_KEY,
    environment: process.env.PINECONE_ENVIRONMENT,
});
// (async () => {
//     await createPineconeIndex(client, indexName, vectorDimension);
// })();

    
const loader = new DirectoryLoader("./data/", {
    ".txt": (path) => new TextLoader(path),
    ".md": (path) => new TextLoader(path),
});

let docs = await loader.load();

const question = "What is fedimint?";

(async () => {
    await updatePinecone(client, indexName, docs);
    await queryPineconeVectorStoreAndQueryLLM(client, indexName, question);
})();
