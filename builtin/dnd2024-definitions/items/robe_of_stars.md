---
splat_schema: 2
splat_id: 6e0e1b04-7b5d-545d-ba2b-5be1adbcc269
splat_kind: item
splat_key: dnd2024:items/robe_of_stars
splat_name: Robe of Stars
splat_origin: dnd2024:reference/magic_item/robe_of_stars
splat_sources:
- SRD 5.2.1, pp. 240, https://www.dndbeyond.com/srd
---

# Robe of Stars


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 240 -->

Wondrous Item, Very Rare (Requires Attunement)

This black or dark-blue robe is embroidered with small white or silver stars. You gain a +1 bonus to saving throws while you wear it.

Six stars, located on the robe’s upper-front portion, are particularly large. While wearing this robe, you can take a Magic action to remove one of the stars and expend it to cast the level 5 version of Magic Missile. Daily at dusk, 1d6 removed stars reappear on the robe.

While you wear the robe, you can take a Magic action to enter the Astral Plane along with everything you are wearing and carrying. You remain there until you take a Magic action to return to the plane you were on. You reappear in the last space you occupied or, if that space is occupied, the nearest unoccupied space.

Original layout: [[../../srd-5.2.1/pages/240|page 240]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/robe_of_stars ` |
| ` implementation ` | ` dnd2024:item_robe_of_stars ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` saving_throw_bonus ` | ` 1 ` |

#### Stars

<!-- splat:yaml path=splat_rule/parameters/stars -->

```yaml
maximum: 6
initial: 6
recovery:
  event: dusk
  amount:
    expression: 1d6
    scope: use
  cap: maximum
```

#### Spell grants

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration | activation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` magic ` | ` item ` | ` 0 ` | ` 5 ` | ` spell_default ` | ` spell_default ` | ` spell_default ` | ` remove_and_expend_star ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/magic_missile ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

#### Astral travel

<!-- splat:values path=splat_rule/parameters/astral_travel -->

| Field | Value |
| --- | --- |
| ` depart_action ` | ` magic ` |
| ` include ` | ` all_worn_and_carried ` |
| ` destination_plane ` | ` astral ` |
| ` duration ` | ` until_return_action ` |
| ` return_action ` | ` magic ` |
| ` return_plane ` | ` prior_plane ` |
| ` return_location ` | ` last_occupied_space ` |
| ` if_occupied ` | ` nearest_unoccupied_space ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
