# Model report for file:///tmp/top-repos-quality-repos-vheepge6/scrapbook_frontend.git HEAD 79cc2b7cdd1bbed3ea47fc15af0bbeea04590f55

### Dump

```json
{'created_at': '2021-09-01 23:14:38',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '22.0 kB',
 'tags': [],
 'uuid': 'c7264689-0f4c-4d37-babd-fe294644ec65',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-vheepge6/scrapbook_frontend.git 79cc2b7cdd1bbed3ea47fc15af0bbeea04590f55

# javascript
16 rules, avg.len. 6.8
## train
PPCR: 0.907655
### report
macro
{'f1-score': 0.2758275812565401,
 'precision': 0.2954046822244971,
 'recall': 0.2658855363690844,
 'support': 16031}
micro
{'f1-score': 0.8586488678186015,
 'precision': 0.8586488678186015,
 'recall': 0.8586488678186015,
 'support': 16031}
weighted
{'f1-score': 0.8363717730078754,
 'precision': 0.8335315641215477,
 'recall': 0.8586488678186015,
 'support': 16031}
### report_full
macro
{'f1-score': 0.2383614297459661,
 'precision': 0.2954046822244971,
 'recall': 0.21223818650416346,
 'support': 17662}
micro
{'f1-score': 0.8170836672305821,
 'precision': 0.8586488678186015,
 'recall': 0.7793568112331559,
 'support': 17662}
weighted
{'f1-score': 0.7732829094472385,
 'precision': 0.793815699520915,
 'recall': 0.7793568112331559,
 'support': 17662}
## test
PPCR: 0.910448
### report
macro
{'f1-score': 0.15880749521035498,
 'precision': 0.16046948356807514,
 'recall': 0.16844091360476662,
 'support': 122}
micro
{'f1-score': 0.6639344262295082,
 'precision': 0.6639344262295082,
 'recall': 0.6639344262295082,
 'support': 122}
weighted
{'f1-score': 0.6197283582478287,
 'precision': 0.6308820133918264,
 'recall': 0.6639344262295082,
 'support': 122}
### report_full
macro
{'f1-score': 0.13607950376842526,
 'precision': 0.16046948356807514,
 'recall': 0.135982905982906,
 'support': 134}
micro
{'f1-score': 0.6328125,
 'precision': 0.6639344262295082,
 'recall': 0.6044776119402985,
 'support': 134}
weighted
{'f1-score': 0.5918117011819446,
 'precision': 0.6595122976665966,
 'recall': 0.6044776119402985,
 'support': 134}
```

## javascript
### Summary
8 rules, avg.len. 6.5

| | |
|-|-|
|Min support|94|
|Max support|696|
|Min confidence|0.95064377784729|
|Max confidence|0.9948979616165161|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  +2.internal_type = CommentBlock<br>⇒ y = ⏎<br>Confidence: 0.982. Support: 696.` |
| 2 | `  -2.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>	∧ +2.internal_type not in {CommentBlock}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 233.` |
| 3 | `  -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {LITERAL}<br>	∧ +2.internal_type not in {CommentBlock}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 101.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 98.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.reserved = =<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 97.` |
| 6 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 228.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INITIALIZATION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 134.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 94.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.5, "max_conf": 0.9948979616165161, "max_support": 696, "min_conf": 0.95064377784729, "min_support": 94, "num_rules": 8}}
```
</details>
