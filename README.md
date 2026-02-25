# Tauri Plugin in-app-review

[查看中文](./README.zh-CN.md)

Allows requesting app ratings within the app, without leaving the current application.

Supported on **iOS** and **Android**.

---

### iOS Policy (StoreKit)

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
  "permissions": ["in-app-review:default"]
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

## Platform Support

- [x] **iOS**: Uses StoreKit APIs
- [x] **Android**: Uses Google Play In-App Review API
- [ ] Desktop platforms (not applicable)

## Android-Specific Notes

The Google Play In-App Review API:

- Only works on Android 5.0 (API level 21) or higher with Google Play Store installed
- Has quota limits to prevent abuse (typically allows showing the dialog a few times per year per user)
- The API doesn't guarantee the dialog will show (depends on quota and Google Play policies)
- Shows a native rating dialog with stars (1-5) and optional comment
- The API doesn't indicate whether the user actually reviewed or if the dialog was shown

### Testing on Android

To test the in-app review flow on Android:

1. Use **Internal App Sharing** or **Internal Test Track** in Google Play Console
2. Install your app through Google Play (not via Android Studio)
3. The review dialog will only appear when installed from Google Play

## Usage (Conceptual Example)

### Testing Behavior

**iOS:**
When your app calls this method while it’s in development mode, StoreKit always displays the rating and review request view, so you can test the user interface and experience. However, this method has no effect in apps that you distribute for beta testing using TestFlight.

**Android:**
The dialog will generally not appear in debug/development builds installed via ADB/Android Studio. You must use the Internal Test Track or Internal App Sharing on the Google Play Console to reliably test the review flow.

```tsx
import { requestReview } from "@gbyte/tauri-plugin-in-app-review";

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

Requests a review for the app.

---

## License

MIT
