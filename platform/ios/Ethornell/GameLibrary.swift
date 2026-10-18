import Foundation

/// Games are folders in the app's Documents (copied there with the Files
/// app or Finder, or imported here); saves live in Application Support,
/// apart from the game files.
final class GameLibrary: ObservableObject {
    @Published private(set) var games: [String] = []

    private let fileManager = FileManager.default

    var documents: URL {
        fileManager.urls(for: .documentDirectory, in: .userDomainMask)[0]
    }

    func refresh() {
        let folders = (try? fileManager.contentsOfDirectory(
            at: documents, includingPropertiesForKeys: [.isDirectoryKey])) ?? []
        games = folders
            .filter { GameLibrary.isGame($0) }
            .map(\.lastPathComponent)
            .sorted { $0.localizedStandardCompare($1) == .orderedAscending }
    }

    /// A folder that holds BGI archives (*.arc).
    static func isGame(_ folder: URL) -> Bool {
        let names = (try? FileManager.default.contentsOfDirectory(atPath: folder.path)) ?? []
        return names.contains { $0.lowercased().hasSuffix(".arc") }
    }

    func gameDirectory(_ game: String) -> URL {
        documents.appendingPathComponent(game, isDirectory: true)
    }

    func userDirectory(_ game: String) -> URL {
        let support = fileManager.urls(for: .applicationSupportDirectory, in: .userDomainMask)[0]
        let directory = support.appendingPathComponent("userdata/\(game)", isDirectory: true)
        try? fileManager.createDirectory(at: directory, withIntermediateDirectories: true)
        return directory
    }

    func isGBK(_ game: String) -> Bool {
        UserDefaults.standard.bool(forKey: "gbk:\(game)")
    }

    func setGBK(_ game: String, _ gbk: Bool) {
        UserDefaults.standard.set(gbk, forKey: "gbk:\(game)")
        objectWillChange.send()
    }

    func delete(_ game: String) {
        try? fileManager.removeItem(at: gameDirectory(game))
        UserDefaults.standard.removeObject(forKey: "gbk:\(game)")
        refresh()
    }

    /// Copies a folder picked in the Files app into Documents.
    func importFolder(_ source: URL) throws {
        let scoped = source.startAccessingSecurityScopedResource()
        defer {
            if scoped { source.stopAccessingSecurityScopedResource() }
        }
        guard GameLibrary.isGame(source) else {
            throw CocoaError(.fileReadCorruptFile, userInfo: [
                NSLocalizedDescriptionKey: NSLocalizedString(
                    "The folder holds no .arc archives.", comment: ""),
            ])
        }
        var target = documents.appendingPathComponent(source.lastPathComponent, isDirectory: true)
        var n = 2
        while fileManager.fileExists(atPath: target.path) {
            target = documents.appendingPathComponent("\(source.lastPathComponent) (\(n))")
            n += 1
        }
        try fileManager.copyItem(at: source, to: target)
        refresh()
    }
}
