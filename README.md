# Tauri Plugin in-app-review

Allows requesting app ratings within the app, without leaving the current application.

> ⚠️ iOS only. This plugin relies on Apple StoreKit APIs and is not available on other platforms.

---

When you call this API in your shipping app and the system displays a rating and review request view, the system handles the entire process for you. Although you normally call this method when it makes sense in the user experience flow of your app, App Store policy governs the actual display of a rating and review request view. When your app calls this API, StoreKit uses the following criteria:

- If the person hasn’t rated or reviewed your app on this device, StoreKit displays the ratings and review request a maximum of three times within a 365-day period.

- If the person has rated or reviewed your app on this device, StoreKit displays the ratings and review request if the app version is new, and if more than 365 days have passed since the person’s previous review.

People can review your app at any time on the App Store. To make it easier for people to leave reviews, you may include a persistent link to your App Store product page in your app’s settings or configuration screens. Append the query parameter action=write-review to your product page URL to automatically open the App Store page where users can write a review.

You can look [Apple Developer Documentation](https://developer.apple.com/documentation/storekit/requestreviewaction)

---

## Installation

```bash
pnpm add @gbyte/tauri-plugin-in-app-review
# or
npm install @gbyte/tauri-plugin-in-app-review
# or
yarn add @gbyte/tauri-plugin-in-app-review
```

Add the plugin to your Tauri project's `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-in-app-review = "0.1"
```

Or use `cargo add tauri-plugin-in-app-review`.

Configure the plugin permissions in your `capabilities/default.json`:

```json
{
  "permissions": [
    "in-app-review:default"
  ]
}
```

Register the plugin in your Tauri app:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_in_app_review::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Usage (Conceptual Example)

When your app calls this method while it’s in development mode, StoreKit always displays the rating and review request view, so you can test the user interface and experience.

However, this method has no effect in apps that you distribute for beta testing using TestFlight.

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
      <main>
        Support this free feature with a quick rating!
      </main>
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

## License

MIT
