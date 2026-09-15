---
splat_schema: 1
splat_id: 6390df81-df8b-5e4f-850a-e6133818db83
splat_kind: item
splat_key: dnd2024:items/talisman_of_ultimate_evil
splat_name: Talisman of Ultimate Evil
splat_origin:
  key: dnd2024:reference/magic_item/talisman_of_ultimate_evil
splat_sources:
- title: SRD 5.2.1
  locator: pp. 249
  url: https://www.dndbeyond.com/srd
---

# Talisman of Ultimate Evil

## Source text

<!-- SRD page 249 -->

Wondrous Item, Legendary (Requires Attunement)\
This item symbolizes unrepentant evil. A creature\
that isn’t a Fiend or an Undead that touches the\
talisman takes 8d6 Necrotic damage and takes the\
damage again each time it ends its turn holding or\
carrying the talisman.\
Holy Symbol. You can use the talisman as a Holy\
Symbol. You gain a +2 bonus to spell attack rolls\
while you wear or hold it.\
Ultimate End. The talisman has 6 charges. While\
wearing or holding the talisman, you can take a\
Magic action to expend 1 charge and target one\
creature you can see on the ground within 120\
feet of yourself. A flaming fissure opens under the\
target, and the target makes a DC 20 Dexterity\
saving throw. If the target is a Celestial, it has Dis-\
advantage on the save. On a failed save, the target\
falls into the fissure and is destroyed, leaving no re-\
mains. On a successful save, the target isn’t cast into\
the fissure but takes 4d6 Psychic damage from the\
ordeal. In either case, the fissure then closes, leav-\
ing no trace of its existence. When you expend the\
last charge, the talisman dissolves into foul-smell-\
ing slime and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/249|page 249]].

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

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/talisman_of_ultimate_evil ` |
| ` implementation ` | ` dnd2024:item_talisman_of_ultimate_evil ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` holy_symbol ` | ` true ` |
| ` attunement_required ` | ` true ` |

#### Touch harm

<!-- splat:yaml path=splat_rule/parameters/touch_harm -->

```yaml
target_type_not_in:
- fiend
- undead
triggers:
- touch
- turn_end_holding_or_carrying
damage:
  amount:
    roll:
      count: 8
      sides: 6
  type: necrotic
requires_attunement: false
```

#### While worn or held

<!-- splat:values path=splat_rule/parameters/while_worn_or_held -->

| Field | Value |
| --- | --- |
| ` spell_attack_bonus ` | ` 2 ` |

#### Charges

<!-- splat:values path=splat_rule/parameters/charges -->

| Field | Value |
| --- | --- |
| ` initial ` | ` 6 ` |
| ` maximum ` | ` 6 ` |
| ` recovery ` | ` null ` |

#### Ultimate end

<!-- splat:yaml path=splat_rule/parameters/ultimate_end -->

```yaml
equipped_any:
- worn
- held
action: magic
charge_cost: 1
target:
  visible: true
  on_ground: true
  range_ft: 120
  creature_count: 1
save:
  ability: dexterity
  dc: 20
  failure:
    destroy_target: true
    remains: false
  success:
    damage:
      amount:
        roll:
          count: 4
          sides: 6
      type: psychic
save_disadvantage_types:
- celestial
fissure: closes_without_trace
```

#### At zero charges

<!-- splat:values path=splat_rule/parameters/at_zero_charges -->

| Field | Value |
| --- | --- |
| ` destroy_item ` | ` true ` |
| ` residue ` | ` slime ` |
