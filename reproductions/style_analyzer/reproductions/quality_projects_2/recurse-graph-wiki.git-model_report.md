# Model report for file:///tmp/top-repos-quality-repos-smy0n1pm/recurse-graph-wiki.git HEAD 849622735ce0aa7ee4ad2e8fc570bc579152edc6

### Dump

```json
{'created_at': '2021-09-02 05:40:30',
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
 'size': '16.2 kB',
 'tags': [],
 'uuid': '439b3922-d24d-4808-84f0-905eeb433418',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-smy0n1pm/recurse-graph-wiki.git 849622735ce0aa7ee4ad2e8fc570bc579152edc6

# javascript
14 rules, avg.len. 5.9
## train
PPCR: 0.746946
### report
macro
{'f1-score': 0.7955623646826283,
 'precision': 0.7939766535459732,
 'recall': 0.7990825945242193,
 'support': 6603}
micro
{'f1-score': 0.9389671361502347,
 'precision': 0.9389671361502347,
 'recall': 0.9389671361502347,
 'support': 6603}
weighted
{'f1-score': 0.9364771153735179,
 'precision': 0.9349540988899622,
 'recall': 0.9389671361502347,
 'support': 6603}
### report_full
macro
{'f1-score': 0.6651838237359089,
 'precision': 0.7939766535459732,
 'recall': 0.5926918041169825,
 'support': 8840}
micro
{'f1-score': 0.8029527941462151,
 'precision': 0.9389671361502347,
 'recall': 0.7013574660633484,
 'support': 8840}
weighted
{'f1-score': 0.7897765313727326,
 'precision': 0.9226670278687936,
 'recall': 0.7013574660633484,
 'support': 8840}
## test
PPCR: 0.790452
### report
macro
{'f1-score': 0.7779043383572253,
 'precision': 0.7719564255059096,
 'recall': 0.7889433381328778,
 'support': 1871}
micro
{'f1-score': 0.9294494922501336,
 'precision': 0.9294494922501336,
 'recall': 0.9294494922501336,
 'support': 1871}
weighted
{'f1-score': 0.923030416007855,
 'precision': 0.9196176665661256,
 'recall': 0.9294494922501336,
 'support': 1871}
### report_full
macro
{'f1-score': 0.6803722015596543,
 'precision': 0.7719564255059096,
 'recall': 0.6281237010593931,
 'support': 2367}
micro
{'f1-score': 0.8206701274185938,
 'precision': 0.9294494922501336,
 'recall': 0.7346852555978032,
 'support': 2367}
weighted
{'f1-score': 0.8042559499471739,
 'precision': 0.912609793680266,
 'recall': 0.7346852555978032,
 'support': 2367}
```

## javascript
### Summary
9 rules, avg.len. 5.7

| | |
|-|-|
|Min support|96|
|Max support|1001|
|Min confidence|0.9475524425506592|
|Max confidence|0.9991087317466736|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 561.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved = '<br>	∧ +1.internal_type = StringLiteral<br>	∧ +4.reserved = ,<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 132.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.reserved = '<br>	∧ +1.internal_type = StringLiteral<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 154.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 967.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.986. Support: 105.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 193.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 182.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 1001.` |
| 9 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 96.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.666666666666667, "max_conf": 0.9991087317466736, "max_support": 1001, "min_conf": 0.9475524425506592, "min_support": 96, "num_rules": 9}}
```
</details>
