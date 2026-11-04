---
splat_schema: 2
splat_id: 8efee93e-b3fd-512d-af09-16db9a747c57
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_narrative_curses
splat_name: Narrative Curses
splat_origin: dnd2024:reference/pages/193
splat_sources:
- SRD 5.2.1, pp. 193, https://www.dndbeyond.com/srd
---

# Narrative Curses

## Source text

<!-- SRD page 193 -->

A curse might manifest during an adventure when a creature’s violation of a taboo warrants supernatural punishment, such as breaking a vow, defiling a tomb, or murdering an innocent. Such a curse can have any effects you design, or it might be a customized version of another type of curse discussed in this section.

A creature affected by such a curse should know why they’re being punished and be able to learn how to end their curse, likely by symbolically righting the wrong they committed. How a spell like Remove Curse affects a curse that’s part of your adventure is up to you—the spell might merely suppress the effects of the curse for a time. Regardless, narrative curses should feel like rare, potent magic rooted in the lore of your campaign.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magical_contagions_narrative_curses ` |
| ` implementation ` | ` dnd2024:rules_magical_contagions_narrative_curses ` |

### Parameters

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm ` |
| ` kind ` | ` taboo_violation ` |
| ` examples ` | ` ["broken_vow", "defiled_tomb", "murdered_innocent"] ` |

#### Curse design

<!-- splat:values path=splat_rule/parameters/curse_design -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm ` |
| ` effect_input_type ` | ` effect ` |
| ` may_customize_other_curse ` | ` true ` |

#### Victim information

<!-- splat:values path=splat_rule/parameters/victim_information -->

| Field | Value |
| --- | --- |
| ` punishment_reason_known ` | ` true ` |
| ` ending_method_discoverable ` | ` true ` |

#### Remove curse response

<!-- splat:values path=splat_rule/parameters/remove_curse_response -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm ` |
| ` mode ` | ` choice ` |
| ` allowed_modes ` | ` ["end", "suppress", "custom_effect"] ` |
| ` suppression_duration_input ` | ` duration ` |
