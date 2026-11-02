---
splat_schema: 1
splat_id: 9a897644-e7f1-55d2-b419-de6018343271
splat_kind: rule
splat_key: dnd2024:glossary/spell_attack
splat_name: Spell Attack
splat_origin:
  key: dnd2024:reference/glossary/spell_attack
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
---

# Spell Attack

## Source text

<!-- SRD page 188 -->

A spell attack is an attack roll made as part of a spell or another magical effect. See also “Spells” (“Casting Spells”).

Original layout: [[../../srd-5.2.1/pages/188|page 188]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/spell_attack ` |
| ` implementation ` | ` dnd2024:glossary_spell_attack ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` test_kind ` | ` attack_roll ` |
| ` source_any ` | ` ["spell", "other_magical_effect"] ` |
