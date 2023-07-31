import { PineconeClient } from "@pinecone-database/pinecone";
import * as dotenv from "dotenv"
import { DirectoryLoader } from 'langchain/document_loaders/fs/directory'
import { TextLoader } from 'langchain/document_loaders/fs/text'
import express  from "express";

import { updatePinecone } from './updatePinecone.js'
import { queryPineconeVectorStoreAndQueryLLM } from "./queryPineconeAndQueryGPT.js"

dotenv.config();

const app = express();
const port = 3338;

const indexName = "fedimint-index"
// const vectorDimension = 1536; // needed for OpenAI embeddings, might be different for other LLMS

const client = new PineconeClient();
await client.init({
    apiKey: process.env.PINECONE_API_KEY,
    environment: process.env.PINECONE_ENVIRONMENT,
});

// (async () => {
//     await createPineconeIndex(client, indexName, vectorDimension);
// })();

    
// const loader = new DirectoryLoader("./data/transcripts", {
//     ".txt": (path) => new TextLoader(path),
//     ".md": (path) => new TextLoader(path),
// });

// let docs = await loader.load();

// const question = "how do you use lightning gateways to turn your federation into a decentralized liquidity provider";
// const answer = await queryPineconeVectorStoreAndQueryLLM(client, indexName, question);
// console.log(answer);

// (async () => {
//     await updatePinecone(client, indexName, docs);
// })();

app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.post('/', async (req, res) => {
    console.log(req.body);
    const question = req.body.question;
    const answer = await queryPineconeVectorStoreAndQueryLLM(client, indexName, question);
    console.log(answer);
    res.send({"answer": answer})
})
app.use(express.static('public'))
app.listen(port, () => {
    console.log(`Example app listening on port ${port}`)
})
