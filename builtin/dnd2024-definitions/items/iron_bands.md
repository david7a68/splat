---
splat_schema: 1
splat_id: 754d424d-1ef9-597c-b4de-d820ef107624
splat_kind: item
splat_key: dnd2024:items/iron_bands
splat_name: Iron Bands
splat_origin:
  key: dnd2024:reference/magic_item/iron_bands
splat_sources:
- title: SRD 5.2.1
  locator: pp. 228
  url: https://www.dndbeyond.com/srd
---

# Iron Bands

## Source text

<!-- SRD page 228 -->

Wondrous Item, Rare\
This rusty iron sphere measures 3 inches in diame-\
ter and weighs 1 pound. You can take a Magic action\
to throw the sphere at a Huge or smaller creature\
you can see within 60 feet of yourself. As the sphere\
moves through the air, it opens into a tangle of\
metal bands.\
Make a ranged attack roll with an attack bonus\
equal to your Dexterity modifier plus your Profi-\
ciency Bonus. On a hit, the target has the Restrained\
condition until you take a Bonus Action to issue a\
command that releases it. Doing so or missing with\
the attack causes the bands to contract and become\
a sphere once more.\
A creature that can touch the bands, including the\
one Restrained, can take an action to make a DC 20\
Strength (Athletics) check to break the iron bands.\
On a successful check, the item is destroyed, and the\
Restrained creature is freed. On a failed check, any\
further attempts made by that creature automati-\
cally fail until 24 hours have elapsed.\
Once the bands are used, they can’t be used again\
until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/228|page 228]].

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
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/iron_bands ` |
| ` implementation ` | ` dnd2024:item_iron_bands ` |

### Parameters

#### Dimensions

<!-- splat:values path=splat_rule/parameters/dimensions -->

| Field | Value |
| --- | --- |
| ` diameter_inches ` | ` 3 ` |
| ` weight_lb ` | ` 1 ` |

#### Activation

<!-- splat:values path=splat_rule/parameters/activation -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` operation ` | ` throw ` |
| ` on_miss ` | ` revert_to_sphere ` |

##### Target

<!-- splat:values path=splat_rule/parameters/activation/target -->

| Field | Value |
| --- | --- |
| ` maximum_size ` | ` huge ` |
| ` seen ` | ` true ` |
| ` range_ft ` | ` 60 ` |

##### Attack

<!-- splat:values path=splat_rule/parameters/activation/attack -->

| Field | Value |
| --- | --- |
| ` kind ` | ` ranged ` |

###### Bonus

###### Sum

<!-- splat:table
path: splat_rule/parameters/activation/attack/bonus/sum
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Value from:
    path: value_from
    type: symbol
-->

| Key | Value from |
| --- | --- |
| ` row_1 ` | ` Dexterity modifier ` |
| ` row_2 ` | ` Proficiency bonus ` |

##### On hit

<!-- splat:values path=splat_rule/parameters/activation/on_hit -->

| Field | Value |
| --- | --- |
| ` condition ` | ` restrained ` |
| ` duration ` | ` until_released_or_broken ` |

##### Resource

<!-- splat:values path=splat_rule/parameters/activation/resource -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 1 ` |
| ` recovery ` | ` dawn ` |

#### Release

<!-- splat:values path=splat_rule/parameters/release -->

| Field | Value |
| --- | --- |
| ` actor ` | ` user ` |
| ` action ` | ` bonus_action ` |
| ` command ` | ` true ` |
| ` effects ` | ` ["end_restraint", "revert_to_sphere"] ` |

#### Break attempt

<!-- splat:yaml path=splat_rule/parameters/break_attempt -->

```yaml
actor: any_creature_touching_bands
action: action
check:
  ability: strength
  skill: athletics
  dc: 20
success:
- destroy_item
- free_creature
failure:
  actor_further_attempts: automatic_failure
  duration:
    value: 24
    unit: hour
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
