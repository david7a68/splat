---
splat_schema: 2
splat_id: 4c34296c-7b48-55e3-8f65-6ce15204d024
splat_kind: item
splat_key: dnd2024:items/boots_of_speed
splat_name: Boots of Speed
splat_origin: dnd2024:reference/magic_item/boots_of_speed
splat_sources:
- SRD 5.2.1, pp. 214, https://www.dndbeyond.com/srd
---

# Boots of Speed


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 214 -->

Wondrous Item, Rare (Requires Attunement)

While you wear these boots, you can take a Bonus Action to click the boots’ heels together. If you do, the boots double your Speed, and any creature that makes an Opportunity Attack against you has Disadvantage on the attack roll. If you click your heels together again, you end the effect.

When you’ve used the boots’ property for a total of 10 minutes, the magic ceases to function for you until you finish a Long Rest.

Original layout: [[../../srd-5.2.1/pages/214|page 214]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/boots_of_speed ` |
| ` implementation ` | ` dnd2024:item_boots_of_speed ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Activation

<!-- splat:values path=splat_rule/parameters/activation -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` gesture ` | ` click_heels ` |
| ` speed_multiplier ` | ` 2 ` |
| ` opportunity_attacks_against ` | ` disadvantage ` |
| ` end_gesture ` | ` click_heels_again ` |

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
active_duration:
  value: 10
  unit: minute
scope: wearer
accumulates_across_activations: true
on_exhaustion: deactivate
recovery: wearer_long_rest
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
