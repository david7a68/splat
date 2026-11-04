---
splat_schema: 2
splat_id: 8ad9c718-ec25-51ed-ac7a-e385f2a232e5
splat_kind: rule
splat_key: dnd2024:glossary/illusions
splat_name: Illusions
splat_origin: dnd2024:reference/glossary/illusions
splat_sources:
- SRD 5.2.1, pp. 183, https://www.dndbeyond.com/srd
---

# Illusions

## Source text

<!-- SRD page 183 -->

Spells and other effects sometimes create magical illusions. Such an effect defines what the illusion does and which senses or mental faculties it deceives.

If an illusion manifests in space, the illusion is insubstantial and weightless, yet it seems to be affected by the environment as if the illusion were real unless the effect that created it specifies otherwise. For example, a visual illusion of a creature casts shadows and reflections, and wind appears to affect the illusory creature. Similarly, an audible illusion echoes in an echoey space.

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/illusions ` |
| ` implementation ` | ` dnd2024:glossary_illusions ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` deceived_senses_and_faculties ` | ` source_effect_defined ` |

#### Spatial manifestation

<!-- splat:values path=splat_rule/parameters/spatial_manifestation -->

| Field | Value |
| --- | --- |
| ` substantial ` | ` false ` |
| ` weight ` | ` 0 ` |
| ` environment_response_appearance ` | ` as_if_real ` |
| ` default_visual_responses ` | ` ["shadows", "reflections", "wind"] ` |
| ` default_audible_responses ` | ` ["echo"] ` |
| ` source_effect_can_override ` | ` true ` |
