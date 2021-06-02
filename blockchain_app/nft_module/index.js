const { BaseModule } = require("lisk-sdk");
const { getAllNFTTokensAsJSON } = require("./nft");

const CreateNFTAsset = require("./transactions/create_nft_asset");
const PurchaseAsset = require("./transactions/purchase_nft_asset");
const TransferNFTAsset = require("./transactions/transfer_nft_asset");

class NFTModule extends BaseModule {
    name = "nft";
    id = 1024;
    accountSchema = {
        type: "object",
        required: ["ownNFTs"],
        properties: {
            ownNFTs: {
                type: "array",
                fieldNumber: 4,
                items: {
                    dataType: "bytes",
                },
            },
        },
        default: {
            ownNFTs: [],
        },
    };

    // add the transaction assets to the module
    transactionAssets = [new CreateNFTAsset(), new PurchaseAsset(), new TransferNFTAsset()];
    actions = {
        // get all the registered NFT tokens from blockchain
        getAllNFTTokens: async () => getAllNFTTokensAsJSON(this._dataAccess), 
    };
}

module.exports = { NFTModule };
