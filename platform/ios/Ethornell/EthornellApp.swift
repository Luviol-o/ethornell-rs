import AVFoundation
import SwiftUI

@main
struct EthornellApp: App {
    init() {
        // Game audio plays with the ring/silent switch, like a media app.
        try? AVAudioSession.sharedInstance().setCategory(.playback)
        try? AVAudioSession.sharedInstance().setActive(true)
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
