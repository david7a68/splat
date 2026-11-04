---
splat_schema: 2
splat_id: 244810ae-b90d-5e81-82bb-418a00484de1
splat_kind: item
splat_key: dnd2024:items/plate_armor_of_etherealness
splat_name: Plate Armor of Etherealness
splat_origin: dnd2024:reference/magic_item/plate_armor_of_etherealness
splat_sources:
- SRD 5.2.1, pp. 234, https://www.dndbeyond.com/srd
---

# Plate Armor of Etherealness


## Item

| Field | Value |
| --- | --- |
| Type | Armor |
| Rarity | Legendary |
| Attunement | Yes |


## Source text

<!-- SRD page 234 -->

Armor (Half Plate Armor or Plate Armor), Legendary (Requires Attunement)

While you’re wearing this armor, you can take a Magic action and use a command word to gain the effect of the Etherealness spell. The spell ends immediately if you remove the armor or take a Magic action to repeat the command word. This property of the armor can’t be used again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/234|page 234]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/plate_armor_of_etherealness ` |
| ` implementation ` | ` dnd2024:item_plate_armor_of_etherealness ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
command_word: true
resource:
  uses: 1
  recovery: dawn
gain_spell_effect:
  spell:
    key: dnd2024:spells/etherealness
end_on:
- armor_removed
- magic_action_repeat_command_word
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
