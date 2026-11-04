---
splat_schema: 2
splat_id: 47a973dc-d6b6-543c-833a-482239b19b4e
splat_kind: item
splat_key: dnd2024:items/demon_armor
splat_name: Demon Armor
splat_origin: dnd2024:reference/magic_item/demon_armor
splat_sources:
- SRD 5.2.1, pp. 218, https://www.dndbeyond.com/srd
---

# Demon Armor


## Item

| Field | Value |
| --- | --- |
| Type | Armor |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 218 -->

Armor (Any Light, Medium, or Heavy), Very Rare (Requires Attunement)

While wearing this armor, you gain a +1 bonus to Armor Class, and you know Abyssal. In addition, the armor’s clawed gauntlets allow your Unarmed Strikes to deal 1d8 Slashing damage instead of the usual Bludgeoning damage, and you gain a +1 bonus to the attack and damage rolls of your Unarmed Strikes.

Curse. Once you don this cursed armor, you can’t doff it unless you are targeted by a Remove Curse spell or similar magic. While wearing the armor, you have Disadvantage on attack rolls against demons and on saving throws against their spells and special abilities.

Original layout: [[../../srd-5.2.1/pages/218|page 218]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/demon_armor ` |
| ` implementation ` | ` dnd2024:item_demon_armor ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` armor_class_bonus ` | ` 1 ` |
| ` languages ` | ` ["abyssal"] ` |

#### Unarmed strike

<!-- splat:yaml path=splat_rule/parameters/unarmed_strike -->

```yaml
damage_dice:
  dice: 1
  sides: 8
  bonus: 0
damage_type: slashing
replaces: normal_bludgeoning_damage
attack_bonus: 1
damage_bonus: 1
```

#### Curse

<!-- splat:values path=splat_rule/parameters/curse -->

| Field | Value |
| --- | --- |
| ` starts ` | ` don_armor ` |
| ` cannot_doff ` | ` true ` |
| ` attack_disadvantage_against ` | ` demons ` |
| ` attunement_required ` | ` false ` |

##### Unlock

<!-- splat:table
path: splat_rule/parameters/curse/unlock
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Spell:
    path: spell
    type: reference
    optional: true
  Magic:
    path: magic
    type: symbol
    optional: true
-->

| Key | Spell | Magic |
| --- | --- | --- |
| ` row_1 ` | [Remove Curse](<../spells/remove_curse.md> "dnd2024:spells/remove_curse") |  |
| ` row_2 ` |  | ` Similar curse removal ` |

##### Save disadvantage against

<!-- splat:values path=splat_rule/parameters/curse/save_disadvantage_against -->

| Field | Value |
| --- | --- |
| ` source ` | ` demons ` |
| ` effect ` | ` ["spells", "special_abilities"] ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
