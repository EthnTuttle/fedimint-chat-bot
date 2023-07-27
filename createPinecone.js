export const createPineconeIndex = async (
    client,
    indexName,
    vectorDimension
) => {
    const existingIndexes = await client.listIndexes();
    if (!existingIndexes.includes(indexName)) {
        console.log(`Creating "${indexName}"...`);
        const createClient = await client.createIndex({
            createRequest: {
                name: indexName,
                dimension: vectorDimension,
                metric: "cosine",
            }
        });
        console.log(`Created with client:`, createClient);
    } else { 
        console.log('index exists')
    }
}