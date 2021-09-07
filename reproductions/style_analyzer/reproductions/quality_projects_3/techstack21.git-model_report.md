# Model report for file:///tmp/top-repos-quality-repos-59oo97sv/techstack21.git HEAD 5f66773939e4d4b0326acb3669d1e6f0e386169c

### Dump

```json
{'created_at': '2021-09-02 07:26:13',
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
 'size': '16.9 kB',
 'tags': [],
 'uuid': '077be114-c78b-4951-9e39-83bbab219131',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-59oo97sv/techstack21.git 5f66773939e4d4b0326acb3669d1e6f0e386169c

# javascript
11 rules, avg.len. 5.0
## train
PPCR: 0.843539
### report
macro
{'f1-score': 0.5690748766281267,
 'precision': 0.5553261102182013,
 'recall': 0.5842160274598,
 'support': 6874}
micro
{'f1-score': 0.9230433517602561,
 'precision': 0.9230433517602561,
 'recall': 0.9230433517602561,
 'support': 6874}
weighted
{'f1-score': 0.9080536303799374,
 'precision': 0.8944497667418388,
 'recall': 0.9230433517602561,
 'support': 6874}
### report_full
macro
{'f1-score': 0.5203457946740765,
 'precision': 0.5553261102182013,
 'recall': 0.4907454346998853,
 'support': 8149}
micro
{'f1-score': 0.8447047859948079,
 'precision': 0.9230433517602561,
 'recall': 0.7786231439440422,
 'support': 8149}
weighted
{'f1-score': 0.8161425655142115,
 'precision': 0.8591060228218762,
 'recall': 0.7786231439440422,
 'support': 8149}
## test
PPCR: 0.842377
### report
macro
{'f1-score': 0.5685627574728389,
 'precision': 0.5573431516384084,
 'recall': 0.5815708283416631,
 'support': 1956}
micro
{'f1-score': 0.9253578732106339,
 'precision': 0.9253578732106339,
 'recall': 0.9253578732106339,
 'support': 1956}
weighted
{'f1-score': 0.9135885236536155,
 'precision': 0.9041351609297862,
 'recall': 0.9253578732106339,
 'support': 1956}
### report_full
macro
{'f1-score': 0.5129368816681892,
 'precision': 0.5573431516384084,
 'recall': 0.48081712620928685,
 'support': 2322}
micro
{'f1-score': 0.8461898083216457,
 'precision': 0.9253578732106339,
 'recall': 0.7795004306632214,
 'support': 2322}
weighted
{'f1-score': 0.8199900514220029,
 'precision': 0.8733757486460503,
 'recall': 0.7795004306632214,
 'support': 2322}
```

## javascript
### Summary
6 rules, avg.len. 3.2

| | |
|-|-|
|Min support|143|
|Max support|1265|
|Min confidence|0.9291845560073853|
|Max confidence|0.9982206225395203|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 1265.` |
| 2 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 143.` |
| 3 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1075.` |
| 4 | `  -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 443.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.reserved = .<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.929. Support: 233.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 281.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.1666666666666665, "max_conf": 0.9982206225395203, "max_support": 1265, "min_conf": 0.9291845560073853, "min_support": 143, "num_rules": 6}}
```
</details>
