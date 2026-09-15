---
splat_schema: 1
splat_id: 47a973dc-d6b6-543c-833a-482239b19b4e
splat_kind: item
splat_key: dnd2024:items/demon_armor
splat_name: Demon Armor
splat_origin:
  key: dnd2024:reference/magic_item/demon_armor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218
  url: https://www.dndbeyond.com/srd
---

# Demon Armor

## Source text

<!-- SRD page 218 -->

Armor (Any Light, Medium, or Heavy), Very Rare\
(Requires Attunement)\
While wearing this armor, you gain a +1 bonus to\
Armor Class, and you know Abyssal. In addition,\
the armor’s clawed gauntlets allow your Unarmed\
Strikes to deal 1d8 Slashing damage instead of the\
usual Bludgeoning damage, and you gain a +1 bonus\
to the attack and damage rolls of your Unarmed\
Strikes.\
Curse. Once you don this cursed armor, you can’t\
doff it unless you are targeted by a Remove Curse\
spell or similar magic. While wearing the armor,\
you have Disadvantage on attack rolls against de-\
mons and on saving throws against their spells and\
special abilities.\

Original layout: [[../../srd-5.2.1/pages/218|page 218]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

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

<!-- splat:yaml path=splat_rule/parameters/curse -->

```yaml
starts: don_armor
cannot_doff: true
unlock:
- spell:
    key: dnd2024:spells/remove_curse
- magic: similar_curse_removal
attack_disadvantage_against: demons
save_disadvantage_against:
  source: demons
  effect:
  - spells
  - special_abilities
attunement_required: false
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
