use std::collections::{HashMap, LinkedList, VecDeque};

/// 页签管理类
pub struct TabManager<T: Eq + std::hash::Hash + Clone> {
    // 所有页签，按插入顺序存储
    all_tabs: VecDeque<T>,
    // 快速查找页签是否存在及其索引
    tab_indices: HashMap<T, usize>,
    // 当前选中的页签
    current_tab: Option<T>,
    // 访问顺序记录（存储的是访问顺序的索引）
    access_order: LinkedList<usize>,
    // 最大显示数量
    max_visible: usize,
}

impl<T: Eq + std::hash::Hash + Clone> TabManager<T> {
    /// 创建一个新的 TabManager，指定最大可见页签数量
    pub fn new(max_visible: usize) -> Self {
        Self {
            all_tabs: VecDeque::new(),
            tab_indices: HashMap::new(),
            current_tab: None,
            access_order: LinkedList::new(),
            max_visible,
        }
    }

    /// 添加或激活一个页签
    pub fn add_or_activate(&mut self, tab: T) {
        if let Some(&index) = self.tab_indices.get(&tab) {
            // 页签已存在，激活它
            self.activate_existing(index);
        } else {
            // 新页签，添加到当前页签后面
            self.add_new(tab);
        }
    }

    /// 删除一个页签
    pub fn remove(&mut self, tab: &T) -> Option<T> {
        let index = match self.tab_indices.get(tab) {
            Some(&i) => i,
            None => return None,
        };

        // 从所有数据结构中移除该页签
        let removed_tab = self.all_tabs.remove(index).unwrap();
        self.tab_indices.remove(tab);

        // 更新索引（因为删除后后面的元素索引会变化）
        for (_, i) in self.tab_indices.iter_mut() {
            if *i > index {
                *i -= 1;
            }
        }

        // 处理访问顺序
        self.access_order = self.access_order.iter()
            .filter_map(|&i| {
                if i == index {
                    None
                } else if i > index {
                    Some(i - 1)
                } else {
                    Some(i)
                }
            })
            .collect();

        // 处理当前页签
        if Some(tab) == self.current_tab.as_ref() {
            self.current_tab = self.access_order.front()
                .and_then(|&i| self.all_tabs.get(i))
                .cloned();
        }

        Some(removed_tab)
    }

    /// 设置最大可见页签数量
    pub fn set_max_visible(&mut self, max_visible: usize) {
        self.max_visible = max_visible;
    }

    /// 获取当前显示的页签列表
    pub fn get_visible_tabs(&self) -> Vec<&T> {
        let visible_count = usize::min(self.max_visible, self.all_tabs.len());
        
        // 收集最近访问的unique索引
        let mut visible_indices: Vec<usize> = self.access_order.iter().take(visible_count).cloned().collect();
        
        // 按插入顺序排序
        visible_indices.sort_unstable();
        
        // 转换为Vec<&T>
        let visible_tabs = visible_indices.iter()
            .filter_map(|&i| self.all_tabs.get(i))
            .collect();
        
        visible_tabs
    }

    /// 获取当前页签
    pub fn current_tab(&self) -> Option<&T> {
        self.current_tab.as_ref()
    }
    
    /// 移动当前页签到指定tab前面(若为空则会放到最后面)
    pub fn move_current_tab(&mut self, anchor_tab: Option<&T>) -> bool {
        let current_tab = match &self.current_tab {
            Some(tab) => tab,
            None => return false,
        };

        let old_index = match self.tab_indices.get(current_tab) {
            Some(&i) => i,
            None => return false,
        };

        let new_index = match anchor_tab {
            Some(tab) => match self.tab_indices.get(tab) {
                Some(&i) => i,
                None => return false,
            }
            None => self.all_tabs.len() - 1,
        };

        if old_index == new_index {
            return false;
        }

        // 移动页签
        let tab = self.all_tabs.remove(old_index).unwrap();
        self.all_tabs.insert(new_index, tab);

        // 更新索引
        self.tab_indices.clear();
        for (i, t) in self.all_tabs.iter().enumerate() {
            self.tab_indices.insert(t.clone(), i);
        }

        // 更新访问顺序中的索引
        self.access_order = self.access_order.iter()
            .map(|&i| {
                if i == old_index {
                    new_index
                } else if i < old_index && i >= new_index {
                    i + 1
                } else if i > old_index && i <= new_index {
                    i - 1
                } else {
                    i
                }
            })
            .collect();

        true
    }

    // 私有方法：激活已存在的页签
    fn activate_existing(&mut self, index: usize) {
        // 更新当前页签
        self.current_tab = self.all_tabs.get(index).cloned();
        
        // 删除访问顺序中的原先索引
        self.access_order = self.access_order.iter()
            .filter(|&&i| i != index)
            .cloned()
            .collect();

        // 更新访问顺序：将当前索引移到最前面
        self.access_order.push_front(index);
    }

    // 私有方法：添加新页签
    fn add_new(&mut self, tab: T) {
        // 确定插入位置（当前页签后面）
        let insert_pos = match &self.current_tab {
            Some(current) => {
                self.tab_indices.get(current).map(|&i| i + 1).unwrap_or(0)
            }
            None => 0,
        };

        // 插入新页签
        self.all_tabs.insert(insert_pos, tab.clone());
        
        // 更新索引
        self.tab_indices.clear();
        for (i, t) in self.all_tabs.iter().enumerate() {
            self.tab_indices.insert(t.clone(), i);
        }
        
        // 更新访问顺序
        for order in self.access_order.iter_mut() {
            if *order >= insert_pos {
                *order += 1;
            } 
        }
        self.access_order.push_front(insert_pos);

        // 设置为当前页签
        self.current_tab = Some(tab);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_activate() {
        let mut manager = TabManager::new(3);
        
        manager.add_or_activate("tab1");
        let tabs = manager.get_visible_tabs();
        let current = manager.current_tab();
        assert_eq!(tabs, vec![&"tab1"]);
        assert_eq!(current, Some(&"tab1"));
        
        manager.add_or_activate("tab2");
        let tabs = manager.get_visible_tabs();
        let current = manager.current_tab();
        assert_eq!(tabs, vec![&"tab1", &"tab2"]);
        assert_eq!(current, Some(&"tab2"));
        
        manager.add_or_activate("tab1");
        let tabs = manager.get_visible_tabs();
        let current = manager.current_tab();
        assert_eq!(tabs, vec![&"tab1", &"tab2"]);
        assert_eq!(current, Some(&"tab1"));
    }

    #[test]
    fn test_remove() {
        let mut manager = TabManager::new(3);
        manager.add_or_activate("tab1");
        manager.add_or_activate("tab2");
        manager.add_or_activate("tab3");
        
        manager.remove(&"tab1");
        let tabs = manager.get_visible_tabs();
        let current = manager.current_tab();
        assert_eq!(tabs, vec![&"tab2", &"tab3"]);
        assert_eq!(current, Some(&"tab3"));
        
        manager.remove(&"tab3");
        let tabs = manager.get_visible_tabs();
        let current = manager.current_tab();
        assert_eq!(tabs, vec![&"tab2"]);
        assert_eq!(current, Some(&"tab2"));
    }

    #[test]
    fn test_max_visible() {
        let mut manager = TabManager::new(2);
        manager.add_or_activate("tab1");
        manager.add_or_activate("tab2");
        manager.add_or_activate("tab3");
        
        let tabs = manager.get_visible_tabs();
        assert_eq!(tabs.len(), 2);
        assert_eq!(tabs, vec![&"tab2", &"tab3"]);
        
        manager.add_or_activate("tab1");
        let tabs = manager.get_visible_tabs();
        assert_eq!(tabs, vec![&"tab1", &"tab3"]);
        
        manager.set_max_visible(3);
        let tabs = manager.get_visible_tabs();
        assert_eq!(tabs.len(), 3);
        assert_eq!(tabs, vec![&"tab1", &"tab2", &"tab3"]);
    }

    #[test]
    fn test_move_current_tab() {
        let mut manager = TabManager::new(5);
        manager.add_or_activate("tab1");
        manager.add_or_activate("tab2");
        manager.add_or_activate("tab3");
        
        // 当前是tab3，移动到第一个位置
        assert!(manager.move_current_tab(Some(&"tab1")));
        let tabs = manager.get_visible_tabs();
        let current = manager.current_tab();
        assert_eq!(tabs, vec![&"tab3", &"tab1", &"tab2"]);
        assert_eq!(current, Some(&"tab3"));
        
        // 激活tab2然后移动
        manager.add_or_activate("tab2");
        assert!(manager.move_current_tab(Some(&"tab1")));
        let tabs = manager.get_visible_tabs();
        let current = manager.current_tab();
        assert_eq!(tabs, vec![&"tab3", &"tab2", &"tab1"]);
        assert_eq!(current, Some(&"tab2"));
        
        // 无效移动
        assert!(!manager.move_current_tab(Some(&"tab4"))); // 不在标签列表中
        assert!(!manager.move_current_tab(Some(&"tab2"))); // 相同位置
    }

    #[test]
    fn test_access_order_memory() {
        let mut manager = TabManager::new(10);
        
        // 添加100个页签
        for i in 0..100 {
            manager.add_or_activate(format!("tab{}", i));
        }
        
        // 访问顺序应该只存储索引，不存储整个字符串
        // 这里我们验证访问顺序的大小不会随着tab内容变大而变大
        let size_before = manager.access_order.len();
        
        // 添加更多内容的大页签
        manager.add_or_activate("a_very_long_tab_name_that_takes_more_memory".to_string());
        
        let size_after = manager.access_order.len();
        assert_eq!(size_before + 1, size_after);
    }
}