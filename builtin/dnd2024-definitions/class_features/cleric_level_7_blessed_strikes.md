---
splat_schema: 1
splat_id: 7ff00766-dac4-5411-9bd6-882b50c54b8d
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_7_blessed_strikes
splat_name: 'Level 7: Blessed Strikes'
splat_origin:
  key: dnd2024:reference/pages/038
splat_sources:
- title: SRD 5.2.1
  locator: pp. 38
  url: https://www.dndbeyond.com/srd
---

# Level 7: Blessed Strikes

## Source text

<!-- SRD page 38 -->

Divine power infuses you in battle. You gain one of the following options of your choice (if you get either option from a Cleric subclass in an older book, use only the option you choose for this feature).

Divine Strike. Once on each of your turns when you hit a creature with an attack roll using a weapon, you can cause the target to take an extra 1d8 Necrotic or Radiant damage (your choice).

Potent Spellcasting. Add your Wisdom modifier to the damage you deal with any Cleric cantrip.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/cleric_level_7_blessed_strikes ` |
| ` implementation ` | ` dnd2024:class_features_cleric_level_7_blessed_strikes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` legacy_subclass_same_options ` | ` only_selected_option_applies ` |

#### Choice

<!-- splat:yaml path=splat_rule/parameters/choice -->

```yaml
count: 1
options:
  divine_strike:
    trigger: actor.weapon_attack_hits_creature
    limit:
      count: 1
      period: actor.turn
    damage:
      dice:
        count: 1
        sides: 8
      type_choice:
      - necrotic
      - radiant
  potent_spellcasting:
    class: cleric
    spell_level: 0
    damage_bonus:
      var: actor.abilities.wisdom.modifier
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` minimum_level ` | ` 7 ` |
