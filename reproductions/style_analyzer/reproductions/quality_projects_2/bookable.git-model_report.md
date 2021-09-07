# Model report for file:///tmp/top-repos-quality-repos-zlnskj7g/bookable.git HEAD 82fe398320ce73f2895f576907461013ba1e454c

### Dump

```json
{'created_at': '2021-09-02 03:19:42',
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
 'size': '14.7 kB',
 'tags': [],
 'uuid': '144ee247-8d01-437a-b4d7-2855241c3289',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-zlnskj7g/bookable.git 82fe398320ce73f2895f576907461013ba1e454c

# javascript
5 rules, avg.len. 4.4
## train
PPCR: 0.535602
### report
macro
{'f1-score': 0.5909228157671499,
 'precision': 0.6206319347846744,
 'recall': 0.5815477511986095,
 'support': 1527}
micro
{'f1-score': 0.9004584151931894,
 'precision': 0.9004584151931893,
 'recall': 0.9004584151931893,
 'support': 1527}
weighted
{'f1-score': 0.8784236891309539,
 'precision': 0.8757479211325008,
 'recall': 0.9004584151931893,
 'support': 1527}
### report_full
macro
{'f1-score': 0.4193408253923609,
 'precision': 0.6206319347846744,
 'recall': 0.3562894848343517,
 'support': 2851}
micro
{'f1-score': 0.6281407035175879,
 'precision': 0.9004584151931893,
 'recall': 0.48228691687127323,
 'support': 2851}
weighted
{'f1-score': 0.5466594327243882,
 'precision': 0.8141357729030566,
 'recall': 0.48228691687127323,
 'support': 2851}
## test
PPCR: 0.546682
### report
macro
{'f1-score': 0.5964935133847095,
 'precision': 0.6058879392212726,
 'recall': 0.6024420024420025,
 'support': 486}
micro
{'f1-score': 0.9176954732510288,
 'precision': 0.9176954732510288,
 'recall': 0.9176954732510288,
 'support': 486}
weighted
{'f1-score': 0.9058112640707241,
 'precision': 0.9093301912231955,
 'recall': 0.9176954732510288,
 'support': 486}
### report_full
macro
{'f1-score': 0.4354705939911172,
 'precision': 0.6058879392212726,
 'recall': 0.38397953610664043,
 'support': 889}
micro
{'f1-score': 0.6487272727272727,
 'precision': 0.9176954732510288,
 'recall': 0.5016872890888638,
 'support': 889}
weighted
{'f1-score': 0.5781737853681364,
 'precision': 0.8614051448697118,
 'recall': 0.5016872890888638,
 'support': 889}
```

## javascript
### Summary
3 rules, avg.len. 4.0

| | |
|-|-|
|Min support|92|
|Max support|273|
|Min confidence|0.9322344064712524|
|Max confidence|0.9950000047683716|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.995. Support: 100.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 273.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 92.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.0, "max_conf": 0.9950000047683716, "max_support": 273, "min_conf": 0.9322344064712524, "min_support": 92, "num_rules": 3}}
```
</details>
