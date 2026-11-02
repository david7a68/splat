---
splat_schema: 1
splat_id: ac881dc0-135f-5b7f-8b1f-5bb10eee10f0
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_communication
splat_name: Communication
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
---

# Communication

## Source text

<!-- SRD page 207 -->

A sentient item communicates by sharing its emotions, broadcasting its thoughts telepathically, or speaking aloud. You can choose how it communicates or roll on the Sentient Item’s Communication table.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sentient_magic_items_communication ` |
| ` implementation ` | ` dnd2024:rules_sentient_magic_items_communication ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` selection ` | ` ["gm_choice", "random_roll"] ` |

#### Die

<!-- splat:values path=splat_rule/parameters/die -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 10 ` |

#### Table

<!-- splat:table
path: splat_rule/parameters/table
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Roll:
    path: ''
    type: range
  Modes:
    path: modes
    type: yaml
  Recipient:
    path: recipient
    type: symbol
    optional: true
  Language count min:
    path: language_count_min
    type: integer
    optional: true
  Languages:
    path: languages
    type: symbol
    optional: true
  Telepathy recipient:
    path: telepathy_recipient
    type: symbol
    optional: true
-->

| Key | Roll | Modes | Recipient | Language count min | Languages | Telepathy recipient |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1–6 ` | ` ["transmit_emotions"] ` | ` Carrying or wielding creature ` |  |  |  |
| ` row_2 ` | ` 7–9 ` | ` ["speech"] ` |  | ` 1 ` | ` Gm choice ` |  |
| ` row_3 ` | ` 10 ` | ` ["speech", "telepathy"] ` |  | ` 1 ` | ` Gm choice ` | ` Carrying or wielding creature ` |
