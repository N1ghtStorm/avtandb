#[cfg(test)]
mod in_memory_graph_tests {
    use std::sync::Arc;
    use std::sync::Mutex;
    use uuid::Uuid;
    use crate::core_model;

    fn initialize_graph_collection() -> core_model::GraphCollectionFacade {
        core_model::GraphCollectionFacade {
            in_memory_graph_collection: Arc::new(Mutex::new(Vec::new()))
        }
    }

    #[test]
    fn validate_and_map_graph_passed() {
        let graph_collection_fac = initialize_graph_collection();
        let dto = core_model::CreateGraphDTO {name: String::from("my_new_graph_name")};
        let result = core_model::validate_and_map_graph(dto, &graph_collection_fac);

        assert_eq!(true, result.is_ok());
        assert_eq!("my_new_graph_name", result.unwrap().name);
    }

    #[test]
    fn validate_and_map_graph_with_filled_passed() {
        let graph_collection_fac = initialize_graph_collection();
        let dto = core_model::CreateGraphDTO {name: String::from("my_new_graph_name")};
 
        {
            let graph_collection_lock = graph_collection_fac.in_memory_graph_collection.lock();
            let mut graph_collection = graph_collection_lock.unwrap();
            graph_collection.push(core_model::InMemoryGraph::new_graph(String::from("some")));
            graph_collection.push(core_model::InMemoryGraph::new_graph(String::from("some2")));
        }

        let result = core_model::validate_and_map_graph(dto, &graph_collection_fac);
        assert_eq!(true, result.is_ok());
        assert_eq!("my_new_graph_name", result.unwrap().name);
    }

    #[test]
    fn validate_and_map_graph_success_failed() {
        //let data = web::Data::new(initialize_graph_collection());
        let graph_collection_fac = initialize_graph_collection();
        let dto = core_model::CreateGraphDTO {name: String::from("my_new_graph_name")};

        {
            let graph_collection_lock = graph_collection_fac.in_memory_graph_collection.lock();
            let mut graph_collection = graph_collection_lock.unwrap();
            graph_collection.push(core_model::InMemoryGraph::new_graph(String::from("some")));
            graph_collection.push(core_model::InMemoryGraph::new_graph(String::from("my_new_graph_name")));
        }

        let result = core_model::validate_and_map_graph(dto, &graph_collection_fac);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn add_node_to_empty_graph_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());
        
        let node = core_model::Node {id: Uuid::default(), labels: vec![String::from("red")]};
        let adding_result = in_mem_graph.add_node(node);

        let node_uuid = in_mem_graph.nodes_collection[0].id;
        let btree_node_id = in_mem_graph.nodes_id_index.get(&node_uuid);

        assert_eq!(0, *btree_node_id.unwrap());
        assert_eq!(true, adding_result.is_ok());
        assert_eq!(1, in_mem_graph.get_nodes_collection_len());
    }

    #[test]
    fn add_node_to_non_empty_graph_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        in_mem_graph.nodes_collection.push(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                        labels: vec![String::from("blue")]});
        in_mem_graph.nodes_collection.push(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                        labels: vec![String::from("green")]});

        let addong_node = core_model::Node {id: Uuid::default(), labels: vec![String::from("red")]};
        let adding_result = in_mem_graph.add_node(addong_node);

        let added_nodes:Vec<Uuid> = in_mem_graph.nodes_collection.iter()
                                                               .filter(|x| x.labels.contains(&String::from("red")))
                                                               .map(|x| x.id)
                                                               .collect();

        let index = added_nodes[0];

        assert_eq!(true, adding_result.is_ok());
        assert_eq!(3, in_mem_graph.get_nodes_collection_len());
        assert_ne!(Uuid::default(), index);
        assert_eq!(1, added_nodes.len());
    }

    #[test]
    fn add_node_to_non_empty_graph_not_zero_id_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let r1 = in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                                labels: vec![String::from("blue")]});
        let r2 = in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                                labels: vec![String::from("green")]});

        let checking_node_uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let adding_node = core_model::Node {id: checking_node_uuid, labels: vec![String::from("red")]};
        let adding_result = in_mem_graph.add_node(adding_node);

        let added_nodes:Vec<Uuid> = in_mem_graph.nodes_collection.iter()
                                                               .filter(|x| x.labels.contains(&String::from("red")))
                                                               .map(|x| x.id)
                                                               .collect();

        let index = added_nodes[0];
        let node_vector_index = in_mem_graph.nodes_id_index.get(&checking_node_uuid);

        assert_eq!(2, *node_vector_index.unwrap());
        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert_eq!(true, adding_result.is_ok());
        assert_eq!(3, in_mem_graph.nodes_collection.len());
        assert_ne!(Uuid::default(), index);
        assert_eq!(1, added_nodes.len());
    }

    #[test]
    fn add_nodes_to_graph_get_correct_index_id_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let r1 = in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                                labels: vec![String::from("blue")]});
        let r2 = in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                                labels: vec![String::from("green")]});
        let r3 = in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400300").unwrap(), 
                                                                labels: vec![String::from("green")]});
        let r4 = in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap(), 
                                                                labels: vec![String::from("green")]});

        let node_vector_index = in_mem_graph.nodes_id_index.get(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap());

        assert_eq!(3, *node_vector_index.unwrap());
        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert!(r3.is_ok());
        assert!(r4.is_ok());
    }

    #[test]
    fn add_node_to_non_empty_graph_id_exists_failed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let r1 = in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                                 labels: vec![String::from("blue")]});
        let r2 = in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                                labels: vec![String::from("green")]});


        let adding_node = core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), labels: vec![String::from("red")]};
        let adding_result = in_mem_graph.add_node(adding_node);

        let is_node_added = in_mem_graph.nodes_collection.iter()
                                                            .any(|x| x.labels.contains(&String::from("red")));
        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert_eq!(true, adding_result.is_err());
        assert_eq!(false, is_node_added);
        assert_eq!(2, in_mem_graph.get_nodes_collection_len());
    }

    #[test]
    fn add_node_blank_label_failed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), 
                                                        labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), 
                                                        labels: vec![String::from("green")]}).unwrap();

        let adding_node = core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap(), labels: vec![String::from("")]};
        let adding_result = in_mem_graph.add_node(adding_node);

        let is_node_added = in_mem_graph.nodes_collection.iter()
                                                            .any(|x| x.labels.contains(&String::from("")));

        assert_eq!(true, adding_result.is_err());
        assert_eq!(false, is_node_added);
        assert_eq!(2, in_mem_graph.get_nodes_collection_len());
    }

    #[test]
    fn add_node_space_label_failed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap(), labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap(), labels: vec![String::from("green")]}).unwrap();

        let adding_node = core_model::Node {id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap(), labels: vec![String::from(" ")]};
        let adding_result = in_mem_graph.add_node(adding_node);

        let is_node_added = in_mem_graph.nodes_collection.iter()
                                                            .any(|x| x.labels.contains(&String::from(" ")));

        assert_eq!(true, adding_result.is_err());
        assert_eq!(false, is_node_added);
        assert_eq!(2, in_mem_graph.get_nodes_collection_len());
    }

    #[test]
    fn add_bonds_to_graph_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_2, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_3, dst: uuid_2, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()}).unwrap();

        let adding_bond = core_model::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_2, id: Uuid::new_v4()};
        let adding_result = in_mem_graph.add_bond(adding_bond);

        assert_eq!(true, adding_result.is_ok());
        assert_eq!(4, in_mem_graph.get_bonds_collection_len());
    }


    #[test]
    fn add_bonds_to_graph_non_existing_node_failed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());
        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();
        
        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        let adding_bond = core_model::Bond {label: String::from("green-green"), src: Uuid::parse_str("550e8400-e29b-41d4-a716-446655400010").unwrap(), 
                                        dst: uuid_2, id: Uuid::new_v4()};
        let adding_result = in_mem_graph.add_bond(adding_bond);

        assert_eq!(true, adding_result.is_err());
        assert_eq!(0, in_mem_graph.get_bonds_collection_len());
    }

    #[test]
    fn add_bonds_to_graph_empty_label_failed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();


        let adding_bond = core_model::Bond {label: String::from(" "), src: uuid_1, dst: uuid_2, id: Uuid::new_v4()};
        let adding_result = in_mem_graph.add_bond(adding_bond);

        assert_eq!(true, adding_result.is_err());
        assert_eq!(0, in_mem_graph.get_bonds_collection_len());
    }


    #[test]
    fn get_simple_connected_nodes_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_2, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_3, dst: uuid_2, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()}).unwrap();

        let connected_nodes_with_2 = in_mem_graph.get_connected_nodes(uuid_2, Vec::new(), Vec::new(),
                                         core_model::BondDirection::Both).unwrap();
        let conn_nodes_ids_with_2: Vec<Uuid> = connected_nodes_with_2.iter().map(|x| x.id).collect();

        assert_eq!(3, connected_nodes_with_2.len());
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap()));
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap()));
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap()));
    }

    #[test]
    fn get_connected_nodes_with_labels_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("grey")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_2, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_3, dst: uuid_2, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()}).unwrap();

        let connected_nodes_with_2 = in_mem_graph.get_connected_nodes(uuid_2, 
                                    Vec::new(), 
                                    vec!["green".to_string()], 
                                    core_model::BondDirection::Both).unwrap();

        let conn_nodes_ids_with_2: Vec<Uuid> = connected_nodes_with_2.iter().map(|x| x.id).collect();

        assert_eq!(2, connected_nodes_with_2.len());
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap()));
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap()));
        assert!(!conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap()));
    }

    #[test]
    fn get_connected_nodes_with_bond_label_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-grey"), src: uuid_2, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_3, dst: uuid_2, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()}).unwrap();

        let connected_nodes_with_2 = in_mem_graph.get_connected_nodes(uuid_2, 
                            vec!["green-green".to_string()],
                                    Vec::new(), 
                                    core_model::BondDirection::Both).unwrap();

        let conn_nodes_ids_with_2: Vec<Uuid> = connected_nodes_with_2.iter().map(|x| x.id).collect();

        assert_eq!(2, connected_nodes_with_2.len());
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap()));
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap()));
        assert!(!conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap()));
    }

    #[test]
    fn get_connected_nodes_with_no_bonds_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        let connected_nodes_with_2 = in_mem_graph.get_connected_nodes(uuid_2, 
                            vec!["green-green".to_string()],
                                    Vec::new(), 
                                    core_model::BondDirection::Both).unwrap();

        let conn_nodes_ids_with_2: Vec<Uuid> = connected_nodes_with_2.iter().map(|x| x.id).collect();

        assert_eq!(1, connected_nodes_with_2.len());
        assert!(conn_nodes_ids_with_2.contains(&Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap()));
    }

    #[test]
    fn get_connected_nodes_with_no_bonds_failed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        let connected_nodes_with_2 = in_mem_graph.get_connected_nodes(Uuid::parse_str("550e8400-e29b-41d4-a716-446655400006").unwrap(), 
                            vec!["green-green".to_string()],
                                    Vec::new(), 
                                    core_model::BondDirection::Both);

        assert!(connected_nodes_with_2.is_err());
    }

    #[test]
    fn get_connected_nodes_with_bond_label_multi_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue1")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("blue2")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("blue3")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("blue4")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue5")]}).unwrap();

        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-grey"), src: uuid_2, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_3, dst: uuid_2, id: Uuid::new_v4()}).unwrap();

        // Add testing bonds to 1
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_1, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster"), src: uuid_1, dst: uuid_3, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster"), src: uuid_1, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster"), src: uuid_1, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster"), src: uuid_1, dst: uuid_2, id: Uuid::new_v4()}).unwrap();
        
        let connected_nodes_with_1 = in_mem_graph.get_connected_nodes(uuid_1, 
                            Vec::new(),
                                    Vec::new(), 
                                    core_model::BondDirection::Both).unwrap();

        let conn_nodes_ids_with_1: Vec<Uuid> = connected_nodes_with_1.iter().map(|x| x.id).collect();

        assert_eq!(8, connected_nodes_with_1.len());
        assert_eq!(8, conn_nodes_ids_with_1.len());
    }

    #[test]
    fn get_connected_nodes_with_bond_label_multi_out_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();


        // Add testing bonds to 1
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_1, dst: uuid_5, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster"), src: uuid_1, dst: uuid_3, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster1"), src: uuid_1, dst: uuid_4, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster2"), src: uuid_1, dst: uuid_2, id: Uuid::new_v4()}).unwrap();
        
        let connected_nodes_with_1 = in_mem_graph.get_connected_nodes(uuid_1, 
                            Vec::new(),
                                    Vec::new(), 
                                    core_model::BondDirection::Both).unwrap();

        let conn_nodes_ids_with_1: Vec<Uuid> = connected_nodes_with_1.iter().map(|x| x.id).collect();

        assert_eq!(5, connected_nodes_with_1.len());
    }

    #[test]
    fn get_connected_nodes_with_bond_label_multi_in_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();

        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        // Add testing bonds to 1
        in_mem_graph.add_bond(core_model::Bond {label: String::from("green-green"), src: uuid_2, dst: uuid_1, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster"), src: uuid_3, dst: uuid_1, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster1"), src: uuid_4, dst: uuid_1, id: Uuid::new_v4()}).unwrap();
        in_mem_graph.add_bond(core_model::Bond {label: String::from("ruster2"), src: uuid_5, dst: uuid_1, id: Uuid::new_v4()}).unwrap();
        
        let connected_nodes_with_1 = in_mem_graph.get_connected_nodes(uuid_1, 
                                    Vec::new(),
                                    Vec::new(), 
                                    core_model::BondDirection::Both).unwrap();

        let conn_nodes_ids_with_1: Vec<Uuid> = connected_nodes_with_1.iter().map(|x| x.id).collect();
        
        assert_eq!(5, connected_nodes_with_1.len());
    }

    #[test]
    fn get_nodes_by_id_list_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();
        let uuid_6 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400006").unwrap();
        let uuid_7 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400007").unwrap();


        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        let id_list = vec![uuid_1, uuid_2, uuid_3, uuid_6, uuid_7];
        let nodes_by_id_list = in_mem_graph.get_nodes_by_id_list(id_list);

        assert_eq!(3, nodes_by_id_list.unwrap().len());
    }

    #[test]
    fn get_nodes_by_id_list_with_dups_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();
        let uuid_6 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400006").unwrap();
        let uuid_7 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400007").unwrap();


        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        let id_list = vec![uuid_1, uuid_1, uuid_2, uuid_2, uuid_3, uuid_6, uuid_7];
        let nodes_by_id_list = in_mem_graph.get_nodes_by_id_list(id_list);

        assert_eq!(3, nodes_by_id_list.unwrap().len());
    }

    #[test]
    fn get_nodes_by_id_list_all_node_exist_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();
        let uuid_6 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400006").unwrap();
        let uuid_7 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400007").unwrap();


        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("blue")]}).unwrap();

        let id_list = vec![uuid_6, uuid_7];
        let nodes_by_id_list = in_mem_graph.get_nodes_by_id_list(id_list);

        assert_eq!(0, nodes_by_id_list.unwrap().len());
    }

    #[test]
    fn get_nodes_by_label_list_passed() {
        let mut in_mem_graph = core_model::InMemoryGraph::new_graph("MyGraph".to_string());

        let uuid_1 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400001").unwrap();
        let uuid_2 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400002").unwrap();
        let uuid_3 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400003").unwrap();
        let uuid_4 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400004").unwrap();
        let uuid_5 = Uuid::parse_str("550e8400-e29b-41d4-a716-446655400005").unwrap();


        in_mem_graph.add_node(core_model::Node {id: uuid_1, labels: vec![String::from("one"), String::from("blue")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_2, labels: vec![String::from("two"), String::from("green")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_3, labels: vec![String::from("three"), String::from("yellow")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_4, labels: vec![String::from("four"), String::from("brown")]}).unwrap();
        in_mem_graph.add_node(core_model::Node {id: uuid_5, labels: vec![String::from("five"), String::from("white")]}).unwrap();

        let label_list = vec![String::from("one"), String::from("yellow"), String::from("five"), String::from("white")];
        let nodes_by_id_list = in_mem_graph.get_nodes_by_label_list(label_list);

        assert_eq!(3, nodes_by_id_list.unwrap().len());
    }
}