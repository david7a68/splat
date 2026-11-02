---
splat_schema: 1
splat_id: 0e90ccf1-865b-5900-81dc-72f525b2b1b8
splat_kind: rule
splat_key: dnd2024:glossary/armor_training
splat_name: Armor Training
splat_origin:
  key: dnd2024:reference/glossary/armor_training
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
---

# Armor Training

## Source text

<!-- SRD page 177 -->

Armor training allows you to use armor of a certain category without the following drawbacks. If you wear Light, Medium, or Heavy armor and lack training with it, you have Disadvantage on any D20 Test that involves Strength or Dexterity, and you can’t cast spells. If you use a Shield and lack training with it, you don’t gain its AC bonus. See also “Disadvantage” and “Equipment” (“Armor”).

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/armor_training ` |
| ` implementation ` | ` dnd2024:glossary_armor_training ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` categories ` | ` ["light", "medium", "heavy", "shield"] ` |

#### Untrained armor

<!-- splat:values path=splat_rule/parameters/untrained_armor -->

| Field | Value |
| --- | --- |
| ` categories ` | ` ["light", "medium", "heavy"] ` |
| ` d20_disadvantage_abilities ` | ` ["strength", "dexterity"] ` |
| ` spellcasting_allowed ` | ` false ` |

#### Untrained shield

<!-- splat:values path=splat_rule/parameters/untrained_shield -->

| Field | Value |
| --- | --- |
| ` shield_ac_bonus ` | ` 0 ` |
