---
splat_schema: 2
splat_id: 2184a1f9-c762-552d-918f-af1845528517
splat_kind: item
splat_key: dnd2024:items/efreeti_bottle
splat_name: Efreeti Bottle
splat_origin: dnd2024:reference/magic_item/efreeti_bottle
splat_sources:
- SRD 5.2.1, pp. 220, https://www.dndbeyond.com/srd
---

# Efreeti Bottle


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Very Rare |
| Attunement | No |


## Source text

<!-- SRD page 220 -->

Wondrous Item, Very Rare

When you take a Magic action to remove the stopper of this painted brass bottle, a cloud of thick smoke flows out of it. At the end of your turn, the smoke disappears with a flash of harmless fire, and an Efreeti appears in an unoccupied space within 30 feet of you.

The first time the bottle is opened, the GM rolls on the following table to determine what happens.

| 1d10 | Effect |
| --- | --- |
| 1 | The efreeti attacks you. After fighting for 5 rounds, the efreeti disappears, and the bottle loses its magic. |
| 2–9 | The efreeti understands your languages and obeys your commands for 1 hour, after which it returns to the bottle, and a new stopper contains it. The stopper can’t be removed for 24 hours. The next two times the bottle is opened, the same effect occurs. If the bottle is opened a fourth time, the efreeti escapes and disappears, and the bottle loses its magic. |
| 10 | The efreeti understands your languages and can cast Wish once for you. It disappears when it grants the wish or after 1 hour, and the bottle loses its magic. |

Original layout: [[../../srd-5.2.1/pages/220|page 220]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/efreeti_bottle ` |
| ` implementation ` | ` dnd2024:item_efreeti_bottle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` activation ` | ` remove_stopper ` |
| ` attunement_required ` | ` false ` |

#### Summon

<!-- splat:yaml path=splat_rule/parameters/summon -->

```yaml
at: actor.turn_end
creature:
  key: dnd2024:creatures/efreeti
space:
  unoccupied: true
  range_ft: 30
```

#### First use roll

<!-- splat:yaml path=splat_rule/parameters/first_use_roll -->

```yaml
die: 10
persistent_result: true
rows:
- range:
  - 1
  - 1
  behavior: attack_user
  duration:
    amount: 5
    unit: rounds
  end:
  - disappear
  - bottle_nonmagical
- range:
  - 2
  - 9
  understands: user_languages
  behavior: obey_commands
  duration:
    amount: 1
    unit: hours
  end:
  - return_to_bottle
  - new_stopper
  stopper_locked:
    amount: 24
    unit: hours
  repeat_on_openings:
  - 2
  - 3
  opening_4:
  - escape
  - disappear
  - bottle_nonmagical
- range:
  - 10
  - 10
  understands: user_languages
  spell:
    spell:
      key: dnd2024:spells/wish
    uses: 1
    beneficiary: user
  ends_on_any:
  - wish_granted
  - one_hour_elapsed
  end:
  - disappear
  - bottle_nonmagical
```
