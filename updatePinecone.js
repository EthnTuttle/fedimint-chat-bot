import { OpenAIEmbeddings } from "langchain/embeddings/openai";
import { RecursiveCharacterTextSplitter } from "langchain/text_splitter";

export const updatePinecone = async (client, indexName, docs) => {
    console.log("Retrieving Pinecone index...");
    const index = client.Index(indexName);
    console.log("index retrieved...")
    for (const doc of docs) {
        console.log(`Processing document: ${doc.metadata.source}`);
        const txtPath = doc.metadata.source;
        const text = doc.pageContent;
        const textSplitter = new RecursiveCharacterTextSplitter({
            chunkSize: 500 // adjust these for varied results, going higher might break token limit on API.
        });
        console.log("Splitting into chunks...")
        const chunks = await textSplitter.createDocuments([text]);
        console.log(`Text split into ${chunks.length} chunks`)
        console.log(`Calling OpenAI's Embedding endpoint documnets with ${chunks.length} text chunks...`);
        const embeddingArrays = await new OpenAIEmbeddings().embedDocuments(
            chunks.map((chunk) => chunk.pageContent.replace(/\n/g, " "))
        );
        console.log("Finished embedding documents.")
        console.log(`Creating ${chunks.length} vectors array with id, values, and metadata...`);
        // create vectors to send to PineCone. Optimal number to upload is 100 accoridng ot Pinecone.
        const batchSize = 100;
        let batch = [];
        for (let idx = 0; idx < chunks.length; idx++ ) {
            const chunk = chunks[idx];
            const vector = {
                id: `${txtPath}_${idx}`,
                values: embeddingArrays[idx],
                metadata: {
                    ...chunk.metadata,
                    loc: JSON.stringify(chunk.metadata.loc),
                    pageContent: chunk.pageContent,
                    txtPath: txtPath
                },
            };
            batch.push(vector);
            if (batch.length === batchSize || idx === chunks.length -1) {
                await index.upsert({
                    upsertRequest: {
                        vectors: batch
                    }
                });
                batch = [];
            }
        }
        console.log(`Pinecone index updates with ${chunks.length} vectors`);
    }
}