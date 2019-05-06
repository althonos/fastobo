

use fastobo::ast as obo;
use horned_owl::model as owl;

use super::Context;
use super::IntoOwlCtx;
use super::OwlEntity;


impl IntoOwlCtx for obo::HeaderClause {
    type Owl = OwlEntity;
    fn into_owl(self, ctx: &mut Context) -> Self::Owl {
        match self {
            // `oboInOwl:hasOBOFormatVersion` annotation
            obo::HeaderClause::FormatVersion(v) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: owl::AnnotationProperty(
                        ctx.build.iri("oboInOwl:hasOBOFormatVersion")
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(v.into_string()),
                        lang: None,
                    })
                }
            ),

            // no equivalent
            // --> should be added as the Ontology IRI
            obo::HeaderClause::DataVersion(_) => OwlEntity::None,

            // `oboInOwl:hasDate` annotation
            // --> QUESTION: should the datatype_iri be `dateTime` or `string` ?
            obo::HeaderClause::Date(dt) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: owl::AnnotationProperty(
                        ctx.build.iri("oboInOwl:hasDate")
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:dateTime")),
                        lang: None,
                        literal: Some(format!(
                            "{:04}-{:02}-{:02}T{:02}:{:02}:00",
                            dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute()
                        ))
                    })
                }
            ),

            // `oboInOwl:savedBy` annotation
            obo::HeaderClause::SavedBy(n) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: owl::AnnotationProperty(
                        ctx.build.iri("oboInOwl:savedBy")
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(n.into_string()),
                        lang: None,
                    })
                }
            ),

            // `oboInOwl:autoGeneratedBy` annotation
            // --> FIXME: not actually declared in `oboInOwl`!
            obo::HeaderClause::AutoGeneratedBy(n) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: owl::AnnotationProperty(
                        ctx.build.iri("oboInOwl:auto-generated-by")
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(n.into_string()),
                        lang: None,
                    })
                }
            ),

            // `owl::imports`:
            // --> if in abbreviated form, use default http://purl.obolibrary.org/obo/ prefix
            // --> if URL, simply use that
            obo::HeaderClause::Import(import) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: owl::AnnotationProperty(
                        ctx.build.iri("owl:imports")
                    ),
                    annotation_value: owl::AnnotationValue::IRI(
                        obo::Url::from(import).into_owl(ctx)
                    )
                }
            ),

            // `owl:AnnotationProperty`
            //     <owl:AnnotationProperty rdf:about=T(subset)>
            //         <rdfs:comment rdf:datatype="xsd:string">T(description)</rdfs:comment>
            //         <rdfs:subPropertyOf rdf:resource="http://www.geneontology.org/formats/oboInOwl#SubsetProperty"/>
            //     </owl:AnnotationProperty>
            obo::HeaderClause::Subsetdef(subset, description) => OwlEntity::Axiom(
                owl::AnnotationAssertion {
                    annotation_subject: obo::Ident::from(subset).into_owl(ctx),
                    annotation: owl::Annotation {
                        annotation_property: owl::AnnotationProperty(
                            ctx.build.iri("rdfs:subPropertyOf")
                        ),
                        annotation_value: owl::AnnotationValue::IRI(
                            ctx.build.iri("oboInOwl:SubsetProperty")
                        )
                    }
                }.into()
            ),

            // `owl:AnnotationProperty`
            //      <owl:AnnotationProperty rdf:about="http://purl.obolibrary.org/obo/go#systematic_synonym">
            //          <oboInOwl:hasScope rdf:resource="http://www.geneontology.org/formats/oboInOwl#hasExactSynonym"/>
            //          <rdfs:label rdf:datatype="http://www.w3.org/2001/XMLSchema#string">Systematic synonym</rdfs:label>
            //          <rdfs:subPropertyOf rdf:resource="http://www.geneontology.org/formats/oboInOwl#SynonymTypeProperty"/>
            //      </owl:AnnotationProperty>
            obo::HeaderClause::SynonymTypedef(ty, desc, scope) => OwlEntity::Axiom(
                owl::AnnotationAssertion {
                    annotation_subject: obo::Ident::from(ty).into_owl(ctx),
                    annotation: owl::Annotation {
                        annotation_property: owl::AnnotationProperty(
                            ctx.build.iri("rdfs:subPropertyOf")
                        ),
                        annotation_value: owl::AnnotationValue::IRI(
                            ctx.build.iri("oboInOwl:SynonymTypeProperty")
                        )
                    }
                }.into()
            ),

            // `oboInOwl:hasDefaultNamespace` annotation
            obo::HeaderClause::DefaultNamespace(ns) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: owl::AnnotationProperty(
                        ctx.build.iri("oboInOwl:hasDefaultNamespace")
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(ns.to_string()),
                        lang: None,
                    })
                }
            ),

            obo::HeaderClause::NamespaceIdRule(r) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: owl::AnnotationProperty(
                        ctx.build.iri("oboInOwl:NamespaceIdRule")
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(r.into_string()),
                        lang: None,
                    })
                }
            ),

            // no equivalent...
            // --> should we use an XML namespace here ?
            obo::HeaderClause::Idspace(_, _, _) => OwlEntity::None,

            // no equivalent, macros should be resolved before conversion.
            obo::HeaderClause::TreatXrefsAsEquivalent(_) => OwlEntity::None,
            obo::HeaderClause::TreatXrefsAsGenusDifferentia(_, _, _) => OwlEntity::None,
            obo::HeaderClause::TreatXrefsAsReverseGenusDifferentia(_, _, _) => OwlEntity::None,
            obo::HeaderClause::TreatXrefsAsRelationship(_, _) => OwlEntity::None,
            obo::HeaderClause::TreatXrefsAsIsA(_) => OwlEntity::None,
            obo::HeaderClause::TreatXrefsAsHasSubclass(_) => OwlEntity::None,

            // `rdfs:comment` annotation
            obo::HeaderClause::Remark(v) => OwlEntity::Annotation(
                owl::Annotation {
                    annotation_property: owl::AnnotationProperty(
                        ctx.build.iri("rdfs:comment")
                    ),
                    annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                        datatype_iri: Some(ctx.build.iri("xsd:string")),
                        literal: Some(v.into_string()),
                        lang: None,
                    })
                }
            ),

            // translate as an annotation
            obo::HeaderClause::PropertyValue(pv) => OwlEntity::Annotation(
                match pv {
                    obo::PropertyValue::Identified(rel, id) => owl::Annotation {
                        annotation_property: owl::AnnotationProperty(obo::Ident::from(rel).into_owl(ctx)),
                        annotation_value: owl::AnnotationValue::IRI(id.into_owl(ctx))
                    },
                    obo::PropertyValue::Typed(rel, value, dty) => owl::Annotation {
                        annotation_property: owl::AnnotationProperty(obo::Ident::from(rel).into_owl(ctx)),
                        annotation_value: owl::AnnotationValue::Literal(owl::Literal {
                            datatype_iri: Some(obo::Ident::from(dty).into_owl(ctx)),
                            literal: Some(value.into_string()),
                            lang: None,
                        })
                    }
                }
            ),

            // no equivalent:
            // --> should be added as the Ontology IRI
            obo::HeaderClause::Ontology(_) => OwlEntity::None,

            // should be added as-is but needs a Manchester-syntax parser
            obo::HeaderClause::OwlAxioms(_) => unimplemented!("cannot translate `owl-axioms` currently"),

            // no equivalent
            // --> FIXME: namespace-id-rule ?
            obo::HeaderClause::Unreserved(_, _) => OwlEntity::None, // FIXME ?
        }
    }
}
