---
splat_schema: 1
splat_id: e9e47748-b495-55bd-af8b-7585b3209dfe
splat_kind: item
splat_key: dnd2024:items/cloak_of_the_bat
splat_name: Cloak of the Bat
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_the_bat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
---

# Cloak of the Bat

## Source text

<!-- SRD page 216 -->

Wondrous Item, Rare (Requires Attunement)

While wearing this cloak, you have Advantage on Dexterity (Stealth) checks. In an area of Dim Light or Darkness, you can grip the edges of the cloak and use it to gain a Fly Speed of 40 feet. If you ever fail to grip the cloak’s edges while flying in this way, or if you are no longer in Dim Light or Darkness, you lose this Fly Speed.

While wearing the cloak in an area of Dim Light or Darkness, you can cast Polymorph on yourself, shape-shifting into a Bat. While in that form, you retain your Intelligence, Wisdom, and Charisma scores. The cloak can’t be used this way again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/216|page 216]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
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
| ` mechanic ` | ` dnd2024:items/cloak_of_the_bat ` |
| ` implementation ` | ` dnd2024:item_cloak_of_the_bat ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Checks

<!-- splat:rows path=splat_rule/parameters/checks -->

| Row | ability | skill | mode |
| --- | --- | --- | --- |
| ` 0 ` | ` dexterity ` | ` stealth ` | ` advantage ` |

#### Flight

<!-- splat:values path=splat_rule/parameters/flight -->

| Field | Value |
| --- | --- |
| ` speed_ft ` | ` 40 ` |
| ` ends_immediately_if_requirement_lost ` | ` true ` |

##### Requires all

<!-- splat:table
path: splat_rule/parameters/flight/requires_all
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Light level in:
    path: light_level_in
    type: yaml
    optional: true
  Gripping cloak edges:
    path: gripping_cloak_edges
    type: boolean
    optional: true
-->

| Key | Light level in | Gripping cloak edges |
| --- | --- | --- |
| ` row_1 ` | ` ["dim_light", "darkness"] ` |  |
| ` row_2 ` |  | true |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | target | retain_ability_scores | activation_requires_light |
| --- | --- | --- | --- |
| ` 0 ` | ` self ` | ` ["intelligence", "wisdom", "charisma"] ` | ` ["dim_light", "darkness"] ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/polymorph ` |

##### Form

<!-- splat:values path=splat_rule/parameters/spells/0/form -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:creatures/bat ` |

##### Uses

<!-- splat:values path=splat_rule/parameters/spells/0/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |
