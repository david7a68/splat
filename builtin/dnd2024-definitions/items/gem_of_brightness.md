---
splat_schema: 2
splat_id: 689f867d-12da-56e3-9b94-78f91528f9bb
splat_kind: item
splat_key: dnd2024:items/gem_of_brightness
splat_name: Gem of Brightness
splat_origin: dnd2024:reference/magic_item/gem_of_brightness
splat_sources:
- SRD 5.2.1, pp. 223, https://www.dndbeyond.com/srd
---

# Gem of Brightness


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 223 -->

Wondrous Item, Uncommon

This prism has 50 charges. While you are holding it, you can take a Magic action and use one of three command words to cause one of the following effects:

First Command Word. The gem sheds Bright Light in a 30-foot radius and Dim Light for an additional 30 feet. This effect doesn’t expend a charge. It lasts until you take a Bonus Action to repeat the command word or until you use another function of the gem.

Second Command Word. You expend 1 charge and cause the gem to fire a brilliant beam of light at one creature you can see within 60 feet of yourself. The creature must succeed on a DC 15 Constitution saving throw or have the Blinded condition for 1 minute. The creature repeats the save at the end of each of its turns, ending the effect on itself on a success.

Third Command Word. You expend 5 charges and cause the gem to flare with intense light in a 30-foot Cone. Each creature in the Cone makes a saving throw as if struck by the beam created with the second command word.

When all of the gem’s charges are expended, the gem becomes a nonmagical jewel worth 50 GP.

Original layout: [[../../srd-5.2.1/pages/223|page 223]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/gem_of_brightness ` |
| ` implementation ` | ` dnd2024:item_gem_of_brightness ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` held ` |
| ` action ` | ` magic ` |
| ` attunement_required ` | ` false ` |

#### Charges

<!-- splat:values path=splat_rule/parameters/charges -->

| Field | Value |
| --- | --- |
| ` initial ` | ` 50 ` |
| ` maximum ` | ` 50 ` |
| ` recovery ` | ` null ` |

#### Command words

<!-- splat:rows path=splat_rule/parameters/command_words -->

| Row | word | charge_cost | ends_on |
| --- | --- | --- | --- |
| ` 0 ` | ` first ` | ` 0 ` | ` ["bonus_action_repeat_first_word", "another_gem_function"] ` |
| ` 1 ` | ` second ` | ` 1 ` |  |
| ` 2 ` | ` third ` | ` 5 ` |  |

##### Light

<!-- splat:values path=splat_rule/parameters/command_words/0/light -->

| Field | Value |
| --- | --- |
| ` bright_radius_ft ` | ` 30 ` |
| ` additional_dim_ft ` | ` 30 ` |

##### Target

<!-- splat:values path=splat_rule/parameters/command_words/1/target -->

| Field | Value |
| --- | --- |
| ` visible ` | ` true ` |
| ` range_ft ` | ` 60 ` |
| ` creature_count ` | ` 1 ` |

##### Save

<!-- splat:yaml path=splat_rule/parameters/command_words/1/save -->

```yaml
ability: constitution
dc: 15
failure:
  condition: blinded
  duration:
    amount: 1
    unit: minutes
repeat:
  event: target.turn_end
  success: end_effect
```

##### Area

<!-- splat:values path=splat_rule/parameters/command_words/2/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cone ` |
| ` size_ft ` | ` 30 ` |

##### Save

<!-- splat:yaml path=splat_rule/parameters/command_words/2/save -->

```yaml
ability: constitution
dc: 15
failure:
  condition: blinded
  duration:
    amount: 1
    unit: minutes
repeat:
  event: target.turn_end
  success: end_effect
```

#### At zero charges

<!-- splat:values path=splat_rule/parameters/at_zero_charges -->

| Field | Value |
| --- | --- |
| ` item_magic ` | ` false ` |
| ` jewel_value_gp ` | ` 50 ` |
