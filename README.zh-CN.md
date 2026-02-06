# Tauri Plugin in-app-review

[English](./README.md)

允许在应用内请求应用评分，而无需离开当前应用程序。

> ⚠️ 仅限 Apple (iOS & macOS)。此插件依赖于 Apple StoreKit API，在其他平台上不可用。

---

当您在发布的应用程序中调用此 API 时，系统会显示一个评分和评论请求视图，并为您处理整个过程。尽管您通常会在应用程序的用户体验流程中适时调用此方法，但 App Store 的政策决定了评分和评论请求视图的实际显示。当您的应用调用此 API 时，StoreKit 会使用以下标准：

- 如果用户尚未在此设备上对您的应用进行评分或评论，StoreKit 将在 365 天内最多显示三次评分和评论请求。

- 如果用户已在此设备上对您的应用进行评分或评论，如果应用版本是新的，并且自用户上次评论以来已超过 365 天，StoreKit 将显示评分和评论请求。

用户可以随时在 App Store 上评论您的应用。为了方便用户留下评论，您可以在应用的设置或配置屏幕中包含一个指向您 App Store 产品页面的永久链接。将查询参数 `action=write-review` 附加到您的产品页面 URL，以自动打开用户可以撰写评论的 App Store 页面。

您可以查看 [Apple 开发者文档](https://developer.apple.com/documentation/storekit/requestreviewaction)

---

## 安装

```bash
pnpm add @gbyte/tauri-plugin-in-app-review
# or
npm install @gbyte/tauri-plugin-in-app-review
# or
yarn add @gbyte/tauri-plugin-in-app-review
```

将插件添加到您的 Tauri 项目的 `Cargo.toml` 中：

```toml
[dependencies]
tauri-plugin-in-app-review = { git = "https://github.com/Gbyte-Group/tauri-plugin-in-app-review", tag = "v0.1.0" }
```

或使用 `cargo add`:

```bash
cargo add --git https://github.com/Gbyte-Group/tauri-plugin-in-app-review tauri-plugin-in-app-review
```

在您的 `src-tauri/capabilities/default.json` 中配置插件权限：

```json
{
  "permissions": ["in-app-review:default"]
}
```

在您的 Tauri 应用中注册插件：

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_in_app_review::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## 用法 (概念示例)

当您的应用在开发模式下调用此方法时，StoreKit 总是会显示评分和评论请求视图，因此您可以测试用户界面和体验。

但是，此方法在您使用 TestFlight 分发测试时无效。

```tsx
import { requestReview } from '@gbyte/tauri-plugin-in-app-review'

export function SuccessModal({ close }) {
  const onSuceess = () => {
    close()
    requestReview()
  }

  return (
    <div>
      <header>
        <h1>Success!</h1>
      </header>
      <main>Support this free feature with a quick rating!</main>
      <footer>
        {/**
         *
         * Because this API may not present an alert, don’t call it in response to a button tap or other user action.
         *
         * Can see https://developer.apple.com/design/human-interface-guidelines/ratings-and-reviews
         *
         */}
        <button onClick={onSuccess}>Yes</button>
        <button onClick={close}>No</button>
      </footer>
    </div>
  )
}
```

---

## APIs

### Methods

#### `requestReview()`

在 app 内请求应用评分。

---

## 许可证

MIT
