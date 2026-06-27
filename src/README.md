# About .panic():
cwte should always panic when it got ANY error, it SHOULD NOT have any recoverable error, we need to make sure the generated code is always correct, but not cwte is always beautiful.    
# About memefd pipeline:
That's just a meme.    
It's just to make sure that we have a dump of each layer, you can use other way to implement it, just make sure one layer do one thing, and upper layer cannot edit lower layer's data, and each layer has a xx_layer.cei for debug build.    
# Symbols:
```
* _CE_SAD :sad path when error
* _CE_HAP :happy path when no error
* _CE_LWE :log when error
* _CE_NUS :just a todo mark
* _CE_LAF :ignore error handler forever
* _CE_DFM :do that for me, an AI-native mark
```