---
splat_schema: 2
splat_id: 281f1de3-6295-5241-b07d-3d5645a13612
splat_kind: item
splat_key: dnd2024:items/cloak_of_arachnida
splat_name: Cloak of Arachnida
splat_origin: dnd2024:reference/magic_item/cloak_of_arachnida
splat_sources:
- SRD 5.2.1, pp. 215, https://www.dndbeyond.com/srd
---

# Cloak of Arachnida


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 215 -->

Wondrous Item, Very Rare (Requires Attunement)

This fine garment is made of black silk interwoven with faint, silvery threads. While wearing it, you gain the following benefits.

Poison Resistance. You have Resistance to Poison damage.

Spider Climb. You have a Climb Speed equal to your Speed and can move up, down, and across vertical surfaces and along ceilings, while leaving your hands free.

Spider Walk. You can’t be caught in webs of any sort and can move through webs as if they were Difficult Terrain.

Web. You can cast Web (save DC 13). The web created by the spell fills twice its normal area. Once used, this property can’t be used again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/215|page 215]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/cloak_of_arachnida ` |
| ` implementation ` | ` dnd2024:item_cloak_of_arachnida ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` resistance ` | ` ["poison"] ` |

#### Climb

<!-- splat:values path=splat_rule/parameters/climb -->

| Field | Value |
| --- | --- |
| ` speed ` | ` walking_speed ` |
| ` vertical_surfaces ` | ` true ` |
| ` ceilings ` | ` true ` |
| ` hands_free ` | ` true ` |

#### Webs

<!-- splat:values path=splat_rule/parameters/webs -->

| Field | Value |
| --- | --- |
| ` cannot_be_caught ` | ` true ` |
| ` movement ` | ` difficult_terrain ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | save_dc | area_multiplier | area_multiplier_scope |
| --- | --- | --- | --- |
| ` 0 ` | ` 13 ` | ` 2 ` | ` web_area ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/web ` |

##### Resource

<!-- splat:values path=splat_rule/parameters/spells/0/resource -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 1 ` |
| ` recovery ` | ` dawn ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
