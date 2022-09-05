
### 准备安装 cli 工具
```
yarn add -D @tauri-apps/cli
cargo install tauri-cli
```

### 创建tauri app
```
yarn create tauri-app --template vue-ts
```

### 测试运行
```
cargo tauri dev
```

### 打包
```
cargo tauri build
```