---
splat_schema: 1
splat_id: 3b4cebb3-bef4-5750-b085-0047a21dc7f1
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_3_potent_cantrip
splat_name: 'Level 3: Potent Cantrip'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
---

# Level 3: Potent Cantrip

## Source text

<!-- SRD page 82 -->

Your damaging cantrips affect even creatures that\
avoid the brunt of the effect. When you cast a can-\
trip at a creature and you miss with the attack roll\
or the target succeeds on a saving throw against the\
cantrip, the target takes half the cantrip’s damage\
(if any) but suffers no additional effect from the\
cantrip.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/evoker_level_3_potent_cantrip ` |
| ` implementation ` | ` dnd2024:class_features_evoker_level_3_potent_cantrip ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` spell_level ` | ` 0 ` |
| ` target_type ` | ` creature ` |
| ` triggers ` | ` ["cantrip_attack_misses", "target_succeeds_cantrip_save"] ` |
| ` damage_multiplier ` | ` 0.5 ` |
| ` additional_cantrip_effects ` | ` false ` |
| ` requires_cantrip_damage ` | ` true ` |
