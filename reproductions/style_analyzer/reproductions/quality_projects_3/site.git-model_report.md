# Model report for file:///tmp/top-repos-quality-repos-asde7398/site.git HEAD bdacc9635bb7d14e74c6a4b754a55465301ef32c

### Dump

```json
{'created_at': '2021-09-02 06:21:19',
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
 'size': '18.9 kB',
 'tags': [],
 'uuid': 'b4eba124-da55-4544-977e-22525cf2ddca',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-asde7398/site.git bdacc9635bb7d14e74c6a4b754a55465301ef32c

# javascript
19 rules, avg.len. 7.2
## train
PPCR: 0.774052
### report
macro
{'f1-score': 0.4534588178649187,
 'precision': 0.462443185347076,
 'recall': 0.45048441350588486,
 'support': 14635}
micro
{'f1-score': 0.9521694567816877,
 'precision': 0.9521694567816877,
 'recall': 0.9521694567816877,
 'support': 14635}
weighted
{'f1-score': 0.9434949515977389,
 'precision': 0.9370523781322702,
 'recall': 0.9521694567816877,
 'support': 14635}
### report_full
macro
{'f1-score': 0.37146528276035085,
 'precision': 0.462443185347076,
 'recall': 0.32091728570379935,
 'support': 18907}
micro
{'f1-score': 0.8308985749209946,
 'precision': 0.9521694567816877,
 'recall': 0.7370286137409425,
 'support': 18907}
weighted
{'f1-score': 0.7948293556197528,
 'precision': 0.8825776278037092,
 'recall': 0.7370286137409425,
 'support': 18907}
## test
PPCR: 0.738565
### report
macro
{'f1-score': 0.40935213535405895,
 'precision': 0.44076129803111835,
 'recall': 0.3967035777557189,
 'support': 3068}
micro
{'f1-score': 0.9367666232073012,
 'precision': 0.9367666232073012,
 'recall': 0.9367666232073012,
 'support': 3068}
weighted
{'f1-score': 0.9293396291933461,
 'precision': 0.9286945761336345,
 'recall': 0.9367666232073012,
 'support': 3068}
### report_full
macro
{'f1-score': 0.3044918170080838,
 'precision': 0.44076129803111835,
 'recall': 0.25299433509269037,
 'support': 4154}
micro
{'f1-score': 0.7959014123511493,
 'precision': 0.9367666232073012,
 'recall': 0.6918632643235436,
 'support': 4154}
weighted
{'f1-score': 0.7517048131296907,
 'precision': 0.8666244155235885,
 'recall': 0.6918632643235436,
 'support': 4154}
```

## javascript
### Summary
14 rules, avg.len. 7.5

| | |
|-|-|
|Min support|91|
|Max support|2495|
|Min confidence|0.9342857003211975|
|Max confidence|0.9987179636955261|

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
                     'min_samples_split': 186,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 2495.` |
| 2 | `  -1.reserved not in {;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 390.` |
| 3 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 460.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.991. Support: 162.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.995. Support: 95.` |
| 6 | `  •••start_col ≤ 29<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 141.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, INCOMPLETE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 1758.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.934. Support: 175.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 325.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 177.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 91.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 344.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2041.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 1672.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.5, "max_conf": 0.9987179636955261, "max_support": 2495, "min_conf": 0.9342857003211975, "min_support": 91, "num_rules": 14}}
```
</details>
