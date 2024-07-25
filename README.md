# Injextism

This repository demonstrates a distributed computing setup using UCANs (User Controller Authenticated Networks) and Extism WASM modules. Followers in the network request and execute a WASM binary upon receiving the appropriate command from the master node. This is strictly for experimetnal purposes and learning. 

Features
- Master-Follower Architecture: A master node sends commands to follower nodes.
- UCAN Authorization: Secure command communication using UCAN tokens.
- Extism WASM Execution: Followers fetch and execute WASM binaries.
- Delegated Masters
