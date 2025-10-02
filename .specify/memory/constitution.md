<!--
Sync Impact Report
==================
Version change: (initial template) → 1.0.0
Modified principles: N/A (initial version)
Added sections:
  - Core Principles (5 principles)
  - Development Standards
  - Governance
Removed sections: N/A
Templates requiring updates:
  ✅ .specify/templates/plan-template.md (Constitution Check section aligns with principles)
  ✅ .specify/templates/spec-template.md (no changes needed - language agnostic)
  ✅ .specify/templates/tasks-template.md (TDD principle alignment confirmed)
Follow-up TODOs:
  - RATIFICATION_DATE set to today (2025-10-02) as initial adoption
-->

# 文書パス管理プロジェクト憲章

## 核となる原則

### I. 日本語ファースト（必須）

すべての文書、コミットメッセージ、AI応答は日本語で記述すること。これはプロジェクトの基本要件であり、例外なく適用される。

- 仕様書（spec.md）、計画書（plan.md）、タスク（tasks.md）等すべての文書は日本語
- Gitコミットメッセージは日本語で記述
- AIエージェントとの対話および応答は日本語
- コード内のコメントは日本語（変数名・関数名は英語可）

**根拠**: プロジェクト関係者の主要言語が日本語であり、コミュニケーション効率と理解精度を最大化するため。

### II. テストファースト（必須）

TDD（テスト駆動開発）を厳格に適用する。すべての実装は「テスト作成 → ユーザー承認 → テスト失敗確認 → 実装」の順序で進める。

- 実装前に必ずテストコードを記述
- テストが失敗することを確認してから実装開始
- Red-Green-Refactorサイクルを厳守
- 契約テスト、統合テスト、単体テストの順で作成

**根拠**: 要件の明確化、リグレッション防止、設計品質の向上を保証するため。

### III. データモデル駆動設計

すべての機能はデータモデル（data-model.md）から出発する。エンティティ、関係性、検証ルールを先に定義し、それに基づいて契約（contracts/）と実装を構築する。

- エンティティ定義が実装の起点
- 状態遷移が必要な場合は明示的に定義
- 検証ルールは要件から抽出し、テスト可能な形式で記述
- データモデルの変更は必ず仕様書（spec.md）にトレース可能であること

**根拠**: データ整合性を保証し、ビジネスロジックの一貫性を維持するため。

### IV. 契約による統合

システム境界（API、CLI、ライブラリ）はすべて契約（OpenAPI/gRPC schema等）で定義する。契約テストを先に作成し、実装は契約に従う。

- 各エンドポイント・コマンドに対して契約定義
- 契約テストは実装前に作成し、必ず失敗させる
- リクエスト/レスポンススキーマの厳密な検証
- 契約変更は後方互換性を考慮（破壊的変更は明示）

**根拠**: インターフェースの明確化、統合の安全性、ドキュメントの自動生成を実現するため。

### V. シンプルさ優先

YAGNI（You Aren't Gonna Need It）原則を適用する。必要になるまで複雑さを導入しない。

- 抽象化は実際の重複が3箇所以上現れてから
- フレームワークやライブラリは必要最小限
- 設計パターンは問題解決に直接寄与する場合のみ使用
- 複雑さの導入は憲章違反として正当化が必要

**根拠**: 保守性を高め、理解コストを下げ、変更容易性を維持するため。

## 開発標準

### 文書構造

- 仕様書: `/specs/[###-feature]/spec.md`（日本語）
- 実装計画: `/specs/[###-feature]/plan.md`（日本語）
- データモデル: `/specs/[###-feature]/data-model.md`（日本語）
- 契約定義: `/specs/[###-feature]/contracts/`
- タスクリスト: `/specs/[###-feature]/tasks.md`（日本語）

### コミットメッセージ規約

日本語で記述し、以下の形式に従う：

```
<type>(<scope>): <subject>

<body>

<footer>
```

Type（日本語または英語プレフィックス）:
- `feat` / `機能`: 新機能
- `fix` / `修正`: バグ修正
- `docs` / `文書`: ドキュメント変更
- `test` / `テスト`: テストコード追加・修正
- `refactor` / `リファクタ`: 機能変更を伴わないコード改善

例:
```
feat(文書パス): 文書種類別パス生成ルール機能を追加

PathGenerationRuleエンティティを実装し、文書種類ごとに
異なる番号フォーマットを定義可能にした。

Closes #42
```

### コードレビュー基準

- 憲章原則の遵守確認（特にI, II, V）
- テストが先に書かれ、失敗していたか
- 日本語コメントの適切性
- データモデルとの整合性
- 契約との整合性

## ガバナンス

### 憲章の優先順位

本憲章はすべての開発プラクティスに優先する。憲章違反のコードはマージ不可。

### 改訂手続き

1. 改訂提案をIssueとして起票（日本語）
2. 影響範囲の分析（どのテンプレート・文書に波及するか）
3. 承認後、憲章バージョンを更新
4. 依存テンプレート（plan-template.md、tasks-template.md等）を同期
5. 移行計画が必要な場合は別途策定

### バージョニング

セマンティックバージョニングに従う：
- **MAJOR**: 後方互換性のない原則の削除・再定義
- **MINOR**: 新原則の追加、セクションの拡張
- **PATCH**: 文言の明確化、タイポ修正

### コンプライアンス

- すべてのPR/レビューで憲章遵守を確認
- 複雑さの導入には正当化を要求（Complexity Trackingセクションに記録）
- 実行時ガイダンスは本憲章に従う

**Version**: 1.0.0 | **Ratified**: 2025-10-02 | **Last Amended**: 2025-10-02
