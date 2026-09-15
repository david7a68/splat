---
splat_schema: 1
splat_id: 561c2107-8c05-5ef6-96cf-f0c76151a8c0
splat_kind: item
splat_key: dnd2024:items/rod_of_rulership
splat_name: Rod of Rulership
splat_origin:
  key: dnd2024:reference/magic_item/rod_of_rulership
splat_sources:
- title: SRD 5.2.1
  locator: pp. 242
  url: https://www.dndbeyond.com/srd
---

# Rod of Rulership

## Source text

<!-- SRD page 242 -->

Rod, Rare (Requires Attunement)\
You can take a Magic action to present the rod and\
command obedience from each creature of your\
choice that you can see within 120 feet of your-\
self. Each target must succeed on a DC 15 Wisdom\
saving throw or have the Charmed condition for 8\
hours. While Charmed in this way, the creature re-\
gards you as its trusted leader. If harmed by you or\
your allies or commanded to do something contrary\
to its nature, a target ceases to be Charmed in this\
way. Once used, this property can’t be used again\
until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/242|page 242]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` rod ` |
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
| ` mechanic ` | ` dnd2024:items/rod_of_rulership ` |
| ` implementation ` | ` dnd2024:item_rod_of_rulership ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` activation ` | ` present_rod_and_command_obedience ` |
| ` attunement_required ` | ` true ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` choice ` | ` user ` |
| ` visible ` | ` true ` |
| ` range_ft ` | ` 120 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc: 15
failure:
  condition: charmed
  duration:
    amount: 8
    unit: hours
  regards_user_as: trusted_leader
  ends_on:
  - harmed_by_user
  - harmed_by_user_ally
  - command_contrary_to_nature
```

#### Uses

<!-- splat:values path=splat_rule/parameters/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |
