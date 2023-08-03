import { PineconeClient } from "@pinecone-database/pinecone";
import * as dotenv from "dotenv"
import { DirectoryLoader } from 'langchain/document_loaders/fs/directory'
import { TextLoader } from 'langchain/document_loaders/fs/text'

import { updatePinecone } from './updatePinecone.js'

dotenv.config();

const indexName = "fedimint-index"

const client = new PineconeClient();
await client.init({
    apiKey: process.env.PINECONE_API_KEY,
    environment: process.env.PINECONE_ENVIRONMENT,
});

    
const loader = new DirectoryLoader("./data/transcripts", {
    ".txt": (path) => new TextLoader(path),
    ".md": (path) => new TextLoader(path),
});

let docs = await loader.load();


(async () => {
    await updatePinecone(client, indexName, docs);
})();
    
    // const question = "how do you use lightning gateways to turn your federation into a decentralized liquidity provider";
    // const answer = await queryPineconeVectorStoreAndQueryLLM(client, indexName, question);
    // console.log(answer);

