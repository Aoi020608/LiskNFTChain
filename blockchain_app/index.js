const { Application, genesisBlockDevnet, configDevnet, HTTPAPIPlugin, utils } = require('lisk-sdk');

// 2 import NFT module and Plugin
const { NFTModule } = require('./nft_module');
const { NFTAPIPlugin } = require('./nft_api_plugin');

genesisBlockDevnet.header.timestamp = 1605699440;
genesisBlockDevnet.header.asset.accounts = genesisBlockDevnet.header.asset.accounts.map(
    (a) =>
        utils.objects.mergeDeep({}, a, {
            nft: {
                ownNFTs: [],
            },
        }),
);

// 4.update application config to include unique label
// and communityIdentifier to mitigate transaction replay
const appConfig = utils.objects.mergeDeep({}, configDevnet, {
    label: 'nft-app',
    genesisConfig: { communityIdentifier: 'NFT' }, // in order to have a unique networkidentifier
    logger: {
        consoleLogLevel: 'info',
    },
});

// 5. Initialize the application with genesis block and application config
const app = Application.defaultApplication(genesisBlockDevnet, appConfig);

// 6. Register custom NFT Module and Plugins
app.registerModule(NFTModule);
app.registerPlugin(HTTPAPIPlugin);
app.registerPlugin(NFTAPIPlugin);

// 7.Run the application
app
    .run()
    .then(() => app.logger.info("NFT Blockchain running..."))
    .catch(console.error);

