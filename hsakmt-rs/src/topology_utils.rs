use std::fs;
use std::path::Path;

pub const KFD_SYSFS_PATH_GENERATION_ID: &str =
    "/sys/devices/virtual/kfd/kfd/topology/generation_id";

pub const KFD_SYSFS_PATH_SYSTEM_PROPERTIES: &str =
    "/sys/devices/virtual/kfd/kfd/topology/system_properties";
pub const KFD_SYSFS_PATH_NODES: &str = "/sys/devices/virtual/kfd/kfd/topology/nodes";

#[derive(Debug, PartialEq)]
pub struct KfdTopologyNodeProperties {
    pub cpu_cores_count: Option<usize>,
    pub simd_count: Option<usize>,
    pub mem_banks_count: Option<usize>,
    pub caches_count: Option<usize>,
    pub io_links_count: Option<usize>,
    pub p2p_links_count: Option<usize>,
    pub cpu_core_id_base: Option<usize>,
    pub simd_id_base: Option<usize>,
    pub capability: Option<usize>,
    pub debug_prop: Option<usize>,
    pub max_waves_per_simd: Option<usize>,
    pub lds_size_in_kb: Option<usize>,
    pub gds_size_in_kb: Option<usize>,
    pub wave_front_size: Option<usize>,
    pub array_count: Option<usize>,
    pub simd_arrays_per_engine: Option<usize>,
    pub cu_per_simd_array: Option<usize>,
    pub simd_per_cu: Option<usize>,
    pub max_slots_scratch_cu: Option<usize>,
    pub fw_version: Option<usize>,
    pub vendor_id: Option<usize>,
    pub device_id: Option<usize>,
    pub location_id: Option<usize>,
    pub domain: Option<usize>,
    pub max_engine_clk_fcompute: Option<usize>,
    pub max_engine_clk_ccompute: Option<usize>,
    pub local_mem_size: Option<usize>,
    pub drm_render_minor: Option<usize>,
    pub sdma_fw_version: Option<usize>,
    pub hive_id: Option<usize>,
    pub unique_id: Option<usize>,
    pub num_sdma_engines: Option<usize>,
    pub num_sdma_xgmi_engines: Option<usize>,
    pub num_gws: Option<usize>,
    pub num_sdma_queues_per_engine: Option<usize>,
    pub num_cp_queues: Option<usize>,
    pub num_xcc: Option<usize>,
    pub gfx_target_version: Option<usize>,
}

#[derive(Debug, PartialEq)]
pub struct KfdTopologyNode {
    pub node_id: usize,
    pub gpu_id: usize,
    pub properties: KfdTopologyNodeProperties,
}

#[derive(Debug, PartialEq)]
pub struct SysDevicesVirtualKfd {
    pub platform_oem: u64,
    pub platform_id: u64,
    pub platform_rev: u64,
    pub nodes: Vec<KfdTopologyNode>,
}

impl SysDevicesVirtualKfd {
    pub fn new() -> Self {
        let mut instance = Self {
            platform_oem: 0,
            platform_id: 0,
            platform_rev: 0,
            nodes: vec![],
        };

        let base_dir = Path::new(KFD_SYSFS_PATH_SYSTEM_PROPERTIES);
        let content = fs::read_to_string(base_dir).unwrap();
        let properties = content.split("\n").collect::<Vec<&str>>();

        for property in properties {
            let pair = property.split(" ").collect::<Vec<&str>>();

            if pair.len() != 2 {
                continue;
            }

            if pair[0] == "platform_oem" {
                instance.platform_oem = pair[1].trim().parse::<u64>().unwrap();
            } else if pair[0] == "platform_id" {
                instance.platform_id = pair[1].trim().parse::<u64>().unwrap();
            } else if pair[0] == "platform_rev" {
                instance.platform_rev = pair[1].trim().parse::<u64>().unwrap();
            }
        }

        instance
    }

    pub fn get_nodes(&self) -> &Vec<KfdTopologyNode> {
        &self.nodes
    }

    pub fn load_nodes(&mut self) {
        let base_dir = Path::new(KFD_SYSFS_PATH_NODES);

        if base_dir.is_dir() {
            for entry in fs::read_dir(base_dir).unwrap() {
                let node_entry_dir = entry.unwrap();
                // println!("{:?}", node_entry_dir);

                let node_id = node_entry_dir
                    .file_name()
                    .to_string_lossy()
                    .to_string()
                    .parse::<usize>()
                    .unwrap();

                let mut kfd_topology_node = KfdTopologyNode {
                    node_id,
                    gpu_id: 0,
                    properties: KfdTopologyNodeProperties {
                        cpu_cores_count: None,
                        simd_count: None,
                        mem_banks_count: None,
                        caches_count: None,
                        io_links_count: None,
                        p2p_links_count: None,
                        cpu_core_id_base: None,
                        simd_id_base: None,
                        capability: None,
                        debug_prop: None,
                        max_waves_per_simd: None,
                        lds_size_in_kb: None,
                        gds_size_in_kb: None,
                        wave_front_size: None,
                        array_count: None,
                        simd_arrays_per_engine: None,
                        cu_per_simd_array: None,
                        simd_per_cu: None,
                        max_slots_scratch_cu: None,
                        fw_version: None,
                        vendor_id: None,
                        device_id: None,
                        location_id: None,
                        domain: None,
                        max_engine_clk_fcompute: None,
                        max_engine_clk_ccompute: None,
                        local_mem_size: None,
                        drm_render_minor: None,
                        sdma_fw_version: None,
                        hive_id: None,
                        unique_id: None,
                        num_sdma_engines: None,
                        num_sdma_xgmi_engines: None,
                        num_gws: None,
                        num_sdma_queues_per_engine: None,
                        num_cp_queues: None,
                        num_xcc: None,
                        gfx_target_version: None,
                    },
                };

                if node_entry_dir.path().is_dir() {
                    for sub_entry in fs::read_dir(node_entry_dir.path()).unwrap() {
                        let node_entry = sub_entry.unwrap();

                        if node_entry.file_name() == "gpu_id" {
                            let gpu_id_str = fs::read_to_string(node_entry.path()).unwrap();
                            kfd_topology_node.gpu_id = gpu_id_str.trim().parse::<usize>().unwrap();
                        }

                        if node_entry.file_name() == "properties" {
                            let content = fs::read_to_string(node_entry.path()).unwrap();
                            let properties = content.split("\n").collect::<Vec<&str>>();

                            for property in properties {
                                let pair = property.split(" ").collect::<Vec<&str>>();

                                if pair.len() != 2 {
                                    continue;
                                }

                                if pair[0] == "cpu_cores_count" {
                                    kfd_topology_node.properties.cpu_cores_count =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "simd_count" {
                                    kfd_topology_node.properties.simd_count =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "mem_banks_count" {
                                    kfd_topology_node.properties.mem_banks_count =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "caches_count" {
                                    kfd_topology_node.properties.caches_count =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "io_links_count" {
                                    kfd_topology_node.properties.io_links_count =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "p2p_links_count" {
                                    kfd_topology_node.properties.p2p_links_count =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "cpu_core_id_base" {
                                    kfd_topology_node.properties.cpu_core_id_base =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "simd_id_base" {
                                    kfd_topology_node.properties.simd_id_base =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "capability" {
                                    kfd_topology_node.properties.capability =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "debug_prop" {
                                    kfd_topology_node.properties.debug_prop =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "max_waves_per_simd" {
                                    kfd_topology_node.properties.max_waves_per_simd =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "lds_size_in_kb" {
                                    kfd_topology_node.properties.lds_size_in_kb =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "gds_size_in_kb" {
                                    kfd_topology_node.properties.gds_size_in_kb =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "wave_front_size" {
                                    kfd_topology_node.properties.wave_front_size =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "array_count" {
                                    kfd_topology_node.properties.array_count =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "simd_arrays_per_engine" {
                                    kfd_topology_node.properties.simd_arrays_per_engine =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "cu_per_simd_array" {
                                    kfd_topology_node.properties.cu_per_simd_array =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "simd_per_cu" {
                                    kfd_topology_node.properties.simd_per_cu =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "max_slots_scratch_cu" {
                                    kfd_topology_node.properties.max_slots_scratch_cu =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "fw_version" {
                                    kfd_topology_node.properties.fw_version =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "vendor_id" {
                                    kfd_topology_node.properties.vendor_id =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "device_id" {
                                    kfd_topology_node.properties.device_id =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "location_id" {
                                    kfd_topology_node.properties.location_id =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "domain" {
                                    kfd_topology_node.properties.domain =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "max_engine_clk_fcompute" {
                                    kfd_topology_node.properties.max_engine_clk_fcompute =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "max_engine_clk_ccompute" {
                                    kfd_topology_node.properties.max_engine_clk_ccompute =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "local_mem_size" {
                                    kfd_topology_node.properties.local_mem_size =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "drm_render_minor" {
                                    kfd_topology_node.properties.drm_render_minor =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "sdma_fw_version" {
                                    kfd_topology_node.properties.sdma_fw_version =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "hive_id" {
                                    kfd_topology_node.properties.hive_id =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "unique_id" {
                                    kfd_topology_node.properties.unique_id =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "num_sdma_engines" {
                                    kfd_topology_node.properties.num_sdma_engines =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "num_sdma_xgmi_engines" {
                                    kfd_topology_node.properties.num_sdma_xgmi_engines =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "num_gws" {
                                    kfd_topology_node.properties.num_gws =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "num_sdma_queues_per_engine" {
                                    kfd_topology_node.properties.num_sdma_queues_per_engine =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "num_cp_queues" {
                                    kfd_topology_node.properties.num_cp_queues =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "num_xcc" {
                                    kfd_topology_node.properties.num_xcc =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                } else if pair[0] == "gfx_target_version" {
                                    kfd_topology_node.properties.gfx_target_version =
                                        Some(pair[1].trim().parse::<usize>().unwrap());
                                }
                            }
                        }
                    }
                }

                self.nodes.push(kfd_topology_node);
            }
        }
    }
}

pub fn num_subdirs(path: &str, text: &str) -> usize {
    let mut count = 0;

    for entry in fs::read_dir(path).unwrap() {
        let node_entry_dir = entry.unwrap();
        let file_name = node_entry_dir.file_name();

        if file_name.to_string_lossy().to_string().contains(text) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysfs_nodes() {
        let mut sys_devices_virtual_kfd = SysDevicesVirtualKfd::new();
        sys_devices_virtual_kfd.load_nodes();

        println!("{:#?}", sys_devices_virtual_kfd);
        // TODO assert
    }

    #[test]
    fn test_num_subdirs() {
        let p = "/sys/devices/system/node/node0";
        let count = num_subdirs(p, "cpu");

        println!("{:#?}", count);
        // TODO assert
    }
}
