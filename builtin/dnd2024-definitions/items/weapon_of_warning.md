---
splat_schema: 2
splat_id: 7d079e82-c907-5bf5-bfb9-5e774668e738
splat_kind: item
splat_key: dnd2024:items/weapon_of_warning
splat_name: Weapon of Warning
splat_origin: dnd2024:reference/magic_item/weapon_of_warning
splat_sources:
- SRD 5.2.1, pp. 253, https://www.dndbeyond.com/srd
---

# Weapon of Warning


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 253 -->

Weapon (Any Simple or Martial), Uncommon (Requires Attunement)

As long as this weapon is within your reach and you are attuned to it, you and allies within 30 feet of you gain the following benefits.

Alarm. The weapon magically awakens each subject who is sleeping naturally when combat begins. This benefit doesn’t wake a subject from magically induced sleep.

Supernatural Readiness. Each subject has Advantage on its Initiative rolls.

Original layout: [[../../srd-5.2.1/pages/253|page 253]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/weapon_of_warning ` |
| ` implementation ` | ` dnd2024:item_weapon_of_warning ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` within_user_reach ` |

#### Aura

<!-- splat:yaml path=splat_rule/parameters/aura -->

```yaml
radius_ft: 30
origin: user
targets:
- self
- allies
combat_start:
  wake_natural_sleep: true
  wake_magical_sleep: false
initiative_advantage: true
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_weapons ` |
