
# 実装計画: 文書パス管理データベース

**ブランチ**: `001-db` | **日付**: 2025-10-02 | **仕様書**: [spec.md](spec.md)
**入力**: `/specs/001-db/spec.md` からの機能仕様書

## 実行フロー（/planコマンドのスコープ）
```
1. 入力パスから機能仕様書を読み込む
   → 見つからない場合: ERROR "No feature spec at {path}"
2. Technical Context を記入（NEEDS CLARIFICATION をスキャン）
   → ファイルシステム構造またはコンテキストからプロジェクトタイプを検出（web=frontend+backend、mobile=app+api）
   → プロジェクトタイプに基づいて構造決定を設定
3. 憲章文書の内容に基づいてConstitution Checkセクションを記入
4. 以下のConstitution Checkセクションを評価
   → 違反が存在する場合: Complexity Trackingに文書化
   → 正当化が不可能な場合: ERROR "Simplify approach first"
   → Progress Trackingを更新: Initial Constitution Check
5. Phase 0 を実行 → research.md
   → NEEDS CLARIFICATIONが残る場合: ERROR "Resolve unknowns"
6. Phase 1 を実行 → contracts、data-model.md、quickstart.md、エージェント固有テンプレートファイル（例: Claude Codeの場合`CLAUDE.md`、GitHub Copilotの場合`.github/copilot-instructions.md`、Gemini CLIの場合`GEMINI.md`、Qwen Codeの場合`QWEN.md`、opencodeの場合`AGENTS.md`）
7. Constitution Checkセクションを再評価
   → 新しい違反がある場合: 設計をリファクタリング、Phase 1に戻る
   → Progress Trackingを更新: Post-Design Constitution Check
8. Phase 2 を計画 → タスク生成アプローチを記述（tasks.mdは作成しない）
9. 停止 - /tasksコマンドの準備完了
```

**重要**: /planコマンドはステップ7で停止します。Phase 2-4は他のコマンドによって実行されます:
- Phase 2: /tasksコマンドがtasks.mdを作成
- Phase 3-4: 実装実行（手動またはツール経由）

## 概要
主要要件: 文書種類ごとに柔軟なパス/番号生成ルールを持ち、照会と論理削除をサポートするDB対応の文書パス管理サービスを提供する。

技術的アプローチ: 既存のRustバックエンド（`axum` + `sqlx` + SQLite）とSvelteフロントエンドを使用。

## 技術的コンテキスト
**言語/バージョン**: Rust 1.90+
**主要依存関係**: axum 0.7（バックエンドWebフレームワーク）、sqlx 0.7（SQLiteデータベース）、tokio 1.35（非同期ランタイム）、Svelte 4.0（フロントエンドフレームワーク）、TypeScript 5.0
**ストレージ**: SQLite（同時読み取りのためのWALモード使用）
**テスト**: cargo test（契約テスト、統合テスト、単体テスト）
**対象プラットフォーム**: Linux/Windowsサーバー（バックエンド）、モダンWebブラウザ（フロントエンド）
**プロジェクトタイプ**: web（バックエンド + フロントエンド）
**パフォーマンス目標**: 文書作成<10ms、照会応答<100ms（FR-014より）
**制約**: 約10,000文書パスを効率的に処理、同時読み取りサポート、書き込み排他制御
**規模/スコープ**: 中規模（約10,000文書パス、複数ユーザー同時アクセス）

## 憲章チェック
*ゲート: Phase 0リサーチ前に合格必須。Phase 1設計後に再チェック。*

**I. 日本語ファースト**
- [x] すべての文書（spec.md、plan.md、tasks.md等）は日本語で記述
- [x] コミットメッセージは日本語
- [x] AIとの対話・応答は日本語

**II. テストファースト**
- [x] 契約テスト、統合テスト、単体テストが実装前に計画されている
- [x] TDDサイクル（Red-Green-Refactor）が設計に組み込まれている

**III. データモデル駆動設計**
- [x] data-model.mdがエンティティ、関係性、検証ルールを定義している
- [x] データモデルがspec.mdの要件にトレース可能

**IV. 契約による統合**
- [x] contracts/ディレクトリにAPI/CLI契約定義が存在
- [x] 契約テストが契約定義に基づいて計画されている

**V. シンプルさ優先**
- [x] YAGNIに従い、必要最小限の複雑さのみ導入
- [x] 複雑さの導入には正当化をComplexity Trackingに記録

## プロジェクト構造

### ドキュメント（この機能）
```
specs/001-db/
├── plan.md              # このファイル（/planコマンドの出力）
├── research.md          # Phase 0 出力（/planコマンド）
├── data-model.md        # Phase 1 出力（/planコマンド）
├── quickstart.md        # Phase 1 出力（/planコマンド）
├── contracts/           # Phase 1 出力（/planコマンド）
└── tasks.md             # Phase 2 出力（/tasksコマンド - /planでは作成されない）
```

### ソースコード（リポジトリルート）
```
backend/
├── src/
│   ├── models/          # データモデル定義
│   ├── services/        # ビジネスロジック
│   └── api/             # HTTPエンドポイント
└── tests/
    ├── contract/        # 契約テスト
    ├── integration/     # 統合テスト
    └── unit/            # 単体テスト

frontend/
├── src/
│   ├── components/      # UIコンポーネント
│   ├── pages/           # ページビュー
│   └── services/        # APIクライアント
└── tests/               # フロントエンドテスト
```

**構造決定**: Webアプリケーション構造（Option 2）を選択。既存のbackend/およびfrontend/ディレクトリを使用。バックエンドはRust（axum + sqlx + SQLite）、フロントエンドはSvelte + TypeScriptで構築。

## Phase 0: 概要とリサーチ
1. **上記Technical Contextから未知事項を抽出**:
   - 各NEEDS CLARIFICATION → リサーチタスク
   - 各依存関係 → ベストプラクティスタスク
   - 各統合 → パターンタスク

2. **リサーチエージェントを生成・派遣**:
   ```
   Technical Contextの各未知事項について:
     タスク: "Research {unknown} for {feature context}"
   各技術選択について:
     タスク: "Find best practices for {tech} in {domain}"
   ```

3. **調査結果を統合** `research.md`に以下の形式で記録:
   - 決定: [選択されたもの]
   - 根拠: [選択理由]
   - 検討した代替案: [他に評価したもの]

**出力**: /home/tagawa/try-spec-kit-via-vs_code/specs/001-db/research.md（すべてのNEEDS CLARIFICATIONが解決済み）

## Phase 1: 設計と契約
*前提条件: research.md完了*

1. **機能仕様書からエンティティを抽出** → `data-model.md`:
   - エンティティ名、フィールド、関係性
   - 要件からの検証ルール
   - 該当する場合は状態遷移

2. **機能要件からAPI契約を生成**:
   - 各ユーザーアクション → エンドポイント
   - 標準的なREST/GraphQLパターンを使用
   - OpenAPI/GraphQLスキーマを`/contracts/`に出力

3. **契約から契約テストを生成**:
   - エンドポイントごとに1つのテストファイル
   - リクエスト/レスポンススキーマをアサート
   - テストは失敗する必要がある（実装はまだ）

4. **ユーザーストーリーからテストシナリオを抽出**:
   - 各ストーリー → 統合テストシナリオ
   - Quickstartテスト = ストーリー検証ステップ

5. **エージェントファイルを段階的に更新**（O(1)操作）:
   - `.specify/scripts/bash/update-agent-context.sh copilot`を実行
     **重要**: 上記で指定されたとおりに実行してください。引数を追加または削除しないでください。
   - 存在する場合: 現在の計画から新しい技術のみを追加
   - マーカー間の手動追加を保持
   - 最近の変更を更新（最後の3つを保持）
   - トークン効率のため150行未満に保つ
   - リポジトリルートに出力

**出力**:
- /home/tagawa/try-spec-kit-via-vs_code/specs/001-db/data-model.md
- /home/tagawa/try-spec-kit-via-vs_code/specs/001-db/contracts/document_service.proto
- /home/tagawa/try-spec-kit-via-vs_code/specs/001-db/quickstart.md
- /home/tagawa/try-spec-kit-via-vs_code/.github/copilot-instructions.md（エージェントファイル更新済み）

## Phase 2: タスク計画アプローチ
*このセクションは/tasksコマンドが実行する内容を記述 - /plan実行中は実行しない*

**タスク生成戦略**:
- `.specify/templates/tasks-template.md`をベースとして読み込む
- Phase 1設計文書（contracts、data model、quickstart）からタスクを生成
- 各契約 → 契約テストタスク [P]
- 各エンティティ → モデル作成タスク [P]
- 各ユーザーストーリー → 統合テストタスク
- テストを合格させるための実装タスク

**順序付け戦略**:
- TDD順序: 実装前にテスト
- 依存関係順序: モデル → サービス → UI
- [P]で並列実行をマーク（独立したファイル）

**推定出力**: tasks.mdに25-30の番号付き、順序付きタスク

**重要**: このフェーズは/tasksコマンドによって実行され、/planでは実行されません

## Phase 3+: 今後の実装
*これらのフェーズは/planコマンドの範囲外*

**Phase 3**: タスク実行（/tasksコマンドがtasks.mdを作成）
**Phase 4**: 実装（憲章原則に従ってtasks.mdを実行）
**Phase 5**: 検証（テスト実行、quickstart.md実行、パフォーマンス検証）

## 複雑さ追跡
*Constitution Checkに正当化が必要な違反がある場合のみ記入*

| 違反 | 必要な理由 | よりシンプルな代替案を却下した理由 |
|-----------|------------|-------------------------------------|
| （該当なし） | - | - |


## 進捗追跡
*このチェックリストは実行フロー中に更新される*

**フェーズステータス**:
 - [x] Phase 0: リサーチ完了（/planコマンド）
 - [x] Phase 1: 設計完了（/planコマンド）
 - [ ] Phase 2: タスク計画完了（/planコマンド - アプローチのみ記述）
 - [ ] Phase 3: タスク生成完了（/tasksコマンド）
 - [ ] Phase 4: 実装完了
 - [ ] Phase 5: 検証合格

**ゲートステータス**:
 - [x] 初回憲章チェック: 合格
 - [x] 設計後憲章チェック: 合格
 - [x] すべてのNEEDS CLARIFICATION解決済み
 - [x] 複雑さの逸脱は文書化済み（該当なし）

---
*憲章 v1.0.0 に基づく - `.specify/memory/constitution.md`を参照*
