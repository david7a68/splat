---
splat_schema: 1
splat_id: 21c77eab-aa16-5c1f-9f60-33cb56973ffb
splat_kind: item
splat_key: dnd2024:items/shield_of_missile_attraction
splat_name: Shield of Missile Attraction
splat_origin:
  key: dnd2024:reference/magic_item/shield_of_missile_attraction
splat_sources:
- title: SRD 5.2.1
  locator: pp. 243
  url: https://www.dndbeyond.com/srd
---

# Shield of Missile Attraction

## Source text

<!-- SRD page 243 -->

Armor (Shield), Rare (Requires Attunement)

While holding this Shield, you have Resistance to damage from attacks made with Ranged weapons.

Curse. This Shield is cursed. Attuning to it curses you until you are targeted by a Remove Curse spell or similar magic. Removing the Shield fails to end the curse on you. Whenever an attack with a Ranged weapon targets a creature within 10 feet of you, the curse causes you to become the target instead.

Original layout: [[../../srd-5.2.1/pages/243|page 243]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/shield_of_missile_attraction ` |
| ` implementation ` | ` dnd2024:item_shield_of_missile_attraction ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_required ` | ` true ` |

#### While held

<!-- splat:yaml path=splat_rule/parameters/while_held -->

```yaml
damage_resistance:
  source: attacks_with_ranged_weapons
```

#### Curse

<!-- splat:values path=splat_rule/parameters/curse -->

| Field | Value |
| --- | --- |
| ` apply_on ` | ` attunement_completed ` |
| ` persists_after_doff ` | ` true ` |

##### Ends on

<!-- splat:table
path: splat_rule/parameters/curse/ends_on
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
  Target:
    path: target
    type: symbol
    optional: true
  Magic equivalent:
    path: magic_equivalent
    type: symbol
    optional: true
  Adjudicator:
    path: adjudicator
    type: symbol
    optional: true
-->

| Key | Spell | Target | Magic equivalent | Adjudicator |
| --- | --- | --- | --- | --- |
| ` row_1 ` | [Remove Curse](<../spells/remove_curse.md> "dnd2024:spells/remove_curse") | ` User ` |  |  |
| ` row_2 ` |  |  | ` Remove curse ` | ` Gm ` |

##### Redirect attack

<!-- splat:values path=splat_rule/parameters/curse/redirect_attack -->

| Field | Value |
| --- | --- |
| ` weapon_category ` | ` ranged ` |
| ` original_target_distance_from_user_ft_max ` | ` 10 ` |
| ` new_target ` | ` user ` |
