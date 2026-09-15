---
splat_schema: 1
splat_id: 43be2844-96d0-5f1b-b02a-7a33082106e2
splat_kind: item
splat_key: dnd2024:items/talisman_of_pure_good
splat_name: Talisman of Pure Good
splat_origin:
  key: dnd2024:reference/magic_item/talisman_of_pure_good
splat_sources:
- title: SRD 5.2.1
  locator: pp. 248, 249
  url: https://www.dndbeyond.com/srd
---

# Talisman of Pure Good

## Source text

<!-- SRD page 248 -->

Wondrous Item, Legendary (Requires Attunement by a\
Cleric or Paladin)\
This talisman is a mighty symbol of goodness. A\
Fiend or an Undead that touches the talisman takes\
8d6 Radiant damage and takes the damage again\
each time it ends its turn holding or carrying the\
talisman.\

<!-- SRD page 249 -->

Holy Symbol. You can use the talisman as a Holy\
Symbol. You gain a +2 bonus to spell attack rolls\
while you wear or hold it.\
Pure Rebuke. The talisman has 7 charges. While\
wearing or holding the talisman, you can take a\
Magic action to expend 1 charge and target one\
creature you can see on the ground within 120 feet\
of yourself. A flaming fissure opens under the tar-\
get, and the target makes a DC 20 Dexterity saving\
throw. If the target is a Fiend or an Undead, it has\
Disadvantage on the save. On a failed save, the tar-\
get falls into the fissure and is destroyed, leaving no\
remains. On a successful save, the target isn’t cast\
into the fissure but takes 4d6 Psychic damage from\
the ordeal. In either case, the fissure then closes,\
leaving no trace of its existence. When you expend\
the last charge, the talisman disperses into motes of\
golden light and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/248|page 248]], [[../../srd-5.2.1/pages/249|page 249]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |
| ` attunement_requirement_text ` | ` a Cleric or Paladin ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/talisman_of_pure_good ` |
| ` implementation ` | ` dnd2024:item_talisman_of_pure_good ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` holy_symbol ` | ` true ` |

#### Touch hazard

<!-- splat:yaml path=splat_rule/parameters/touch_hazard -->

```yaml
attunement_required: false
target_types:
- fiend
- undead
triggers:
- touch_item
- end_turn_holding_or_carrying_item
damage:
  roll:
    dice: 8
    sides: 6
    bonus: 0
  type: radiant
```

#### Held or worn

<!-- splat:values path=splat_rule/parameters/held_or_worn -->

| Field | Value |
| --- | --- |
| ` spell_attack_bonus ` | ` 2 ` |

#### Charges

<!-- splat:values path=splat_rule/parameters/charges -->

| Field | Value |
| --- | --- |
| ` initial ` | ` 7 ` |
| ` maximum ` | ` 7 ` |
| ` recovery ` | ` null ` |
| ` on_zero ` | ` destroy_item ` |

#### Pure rebuke

<!-- splat:yaml path=splat_rule/parameters/pure_rebuke -->

```yaml
requires: held_or_worn
action: magic
cost: 1
target:
  kind: creature
  count: 1
  seen: true
  on_ground: true
  range_ft: 120
save:
  ability: dexterity
  dc: 20
  disadvantage_types:
  - fiend
  - undead
failure:
  fall_into_fissure: true
  destroy_target: true
  remains: false
success:
  damage:
    roll:
      dice: 4
      sides: 6
      bonus: 0
    type: psychic
  fall_into_fissure: false
after_either_result:
  fissure: closes_without_trace
```

#### Attunement

<!-- splat:yaml path=splat_rule/parameters/attunement -->

```yaml
required: true
eligibility:
  class_any:
  - cleric
  - paladin
```
