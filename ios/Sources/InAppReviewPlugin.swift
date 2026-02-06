import StoreKit
import SwiftRs
import Tauri
import UIKit
import WebKit

class InAppReviewPlugin: Plugin {
  @objc public func requestReview(_ invoke: Invoke) {
    DispatchQueue.main.async {
      if let scene = UIApplication.shared.connectedScenes
        .first(where: { $0.activationState == .foregroundActive }) as? UIWindowScene
      {
        if #available(iOS 18, *) {
          AppStore.requestReview(in: scene)
        } else if #available(iOS 14, *) {
          SKStoreReviewController.requestReview(in: scene)
        } else {
          SKStoreReviewController.requestReview()
        }
      } else {
        invoke.reject("not found scene")
      }
    }
  }
}

@_cdecl("init_plugin_in_app_review")
func initPlugin() -> Plugin {
  return InAppReviewPlugin()
}
