import SwiftUI
import UniformTypeIdentifiers

struct PlayingGame: Identifiable {
    let name: String
    var id: String { name }
}

/// The game list: import a folder, pick a game, set its text encoding.
struct ContentView: View {
    @StateObject private var library = GameLibrary()
    @State private var importing = false
    @State private var playing: PlayingGame?
    @State private var errorMessage: String?

    var body: some View {
        NavigationView {
            List {
                Section(footer: Text("Copy game folders (with their .arc files) into Ethornell with the Files app or Finder, or import one.")) {
                    ForEach(library.games, id: \.self) { game in
                        Button {
                            playing = PlayingGame(name: game)
                        } label: {
                            HStack {
                                Text(game).foregroundColor(.primary)
                                Spacer()
                                Text(library.isGBK(game) ? "GBK" : "Shift-JIS")
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                        }
                        .contextMenu {
                            Button("Text: Shift-JIS (Japanese)") { library.setGBK(game, false) }
                            Button("Text: GBK (Chinese translation)") { library.setGBK(game, true) }
                            Button("Delete", role: .destructive) { library.delete(game) }
                        }
                    }
                }
            }
            .navigationTitle("Ethornell")
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button {
                        importing = true
                    } label: {
                        Label("Import game folder", systemImage: "folder.badge.plus")
                    }
                }
            }
            .fileImporter(isPresented: $importing, allowedContentTypes: [.folder]) { result in
                do {
                    try library.importFolder(result.get())
                } catch {
                    errorMessage = error.localizedDescription
                }
            }
            .alert(item: Binding(
                get: { errorMessage.map { PlayingGame(name: $0) } },
                set: { errorMessage = $0?.name }
            )) { message in
                Alert(title: Text("Import failed"), message: Text(message.name))
            }
            .refreshable { library.refresh() }
        }
        .navigationViewStyle(.stack)
        .onAppear { library.refresh() }
        .onReceive(NotificationCenter.default.publisher(
            for: UIApplication.willEnterForegroundNotification)) { _ in library.refresh() }
        .fullScreenCover(item: $playing) { game in
            GameScreen(library: library, game: game.name) { playing = nil }
                .ignoresSafeArea()
                .statusBarHidden(true)
        }
    }
}

struct GameScreen: UIViewControllerRepresentable {
    let library: GameLibrary
    let game: String
    let onClose: () -> Void

    func makeUIViewController(context: Context) -> GameViewController {
        GameViewController(
            resourceRoot: library.gameDirectory(game),
            userRoot: library.userDirectory(game),
            gbk: library.isGBK(game),
            onClose: onClose)
    }

    func updateUIViewController(_ controller: GameViewController, context: Context) {}
}
