---
splat_schema: 1
splat_id: 0c7ef23c-412e-528a-8d62-9deb5250fc57
splat_kind: item
splat_key: dnd2024:items/broom_of_flying
splat_name: Broom of Flying
splat_origin:
  key: dnd2024:reference/magic_item/broom_of_flying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
---

# Broom of Flying

## Source text

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)\
This wooden broom functions like a mundane\
broom until you stand astride it and take a Magic\
action to make it hover beneath you, at which time\
it can be ridden in the air. It has a Fly Speed of 50\
feet. It can carry up to 400 pounds, but its Fly Speed\
becomes 30 feet while carrying over 200 pounds.\
The broom stops hovering when you land or when\
you’re no longer riding it.\
As a Magic action, you can send the broom to\
travel alone to a destination within 1 mile of you if\
you name the location and are familiar with it. The\
broom comes back to you when you take a Magic\
action and use a command word if the broom is still\
within 1 mile of you.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/broom_of_flying ` |
| ` implementation ` | ` dnd2024:item_broom_of_flying ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` default_mode ` | ` mundane_broom ` |
| ` fly_speed_ft ` | ` 50 ` |
| ` capacity_lb ` | ` 400 ` |
| ` hover_ends_on ` | ` ["landing", "dismounted"] ` |
| ` attunement_required ` | ` true ` |

#### Activate

<!-- splat:values path=splat_rule/parameters/activate -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` must_stand_astride ` | ` true ` |
| ` mode ` | ` hovering_ridable ` |

#### Encumbered

<!-- splat:values path=splat_rule/parameters/encumbered -->

| Field | Value |
| --- | --- |
| ` over_lb ` | ` 200 ` |
| ` fly_speed_ft ` | ` 30 ` |

#### Send

<!-- splat:yaml path=splat_rule/parameters/send -->

```yaml
action: magic
destination:
  familiar: true
  named: true
  maximum_distance_miles: 1
travels_alone: true
```

#### Recall

<!-- splat:values path=splat_rule/parameters/recall -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` command_word ` | ` true ` |
| ` maximum_distance_miles ` | ` 1 ` |
| ` destination ` | ` actor ` |
