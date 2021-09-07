# Model report for file:///tmp/top-repos-quality-repos-nu_6hocy/bot.git HEAD 010b1cd4cb62eef8e907f69a36c1752c5cb0bae8

### Dump

```json
{'created_at': '2021-09-02 06:06:55',
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
 'size': '17.1 kB',
 'tags': [],
 'uuid': 'e56b0b2b-8d14-4f04-b9b0-0e1cf8cdf613',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-nu_6hocy/bot.git 010b1cd4cb62eef8e907f69a36c1752c5cb0bae8

# javascript
46 rules, avg.len. 7.7
## train
PPCR: 0.968677
### report
macro
{'f1-score': 0.9087292455144087,
 'precision': 0.9465501240700276,
 'recall': 0.888018239524125,
 'support': 90334}
micro
{'f1-score': 0.9763322779905683,
 'precision': 0.9763322779905683,
 'recall': 0.9763322779905683,
 'support': 90334}
weighted
{'f1-score': 0.9758336027255398,
 'precision': 0.9765439407133842,
 'recall': 0.9763322779905683,
 'support': 90334}
### report_full
macro
{'f1-score': 0.856976288571073,
 'precision': 0.9465501240700276,
 'recall': 0.8122150867731845,
 'support': 93255}
micro
{'f1-score': 0.9607983049093355,
 'precision': 0.9763322779905683,
 'recall': 0.9457508980751702,
 'support': 93255}
weighted
{'f1-score': 0.9581818625291716,
 'precision': 0.9753647296538782,
 'recall': 0.9457508980751702,
 'support': 93255}
## test
PPCR: 0.961043
### report
macro
{'f1-score': 0.8579571934747703,
 'precision': 0.8784456194357154,
 'recall': 0.8479008184954969,
 'support': 20204}
micro
{'f1-score': 0.9622352009503069,
 'precision': 0.9622352009503069,
 'recall': 0.9622352009503069,
 'support': 20204}
weighted
{'f1-score': 0.9621912330094697,
 'precision': 0.9641669076432215,
 'recall': 0.9622352009503069,
 'support': 20204}
### report_full
macro
{'f1-score': 0.8089400383651516,
 'precision': 0.8784456194357154,
 'recall': 0.774816812606285,
 'support': 21023}
micro
{'f1-score': 0.9431198001309822,
 'precision': 0.9622352009503069,
 'recall': 0.9247490843362032,
 'support': 21023}
weighted
{'f1-score': 0.9404258947338899,
 'precision': 0.9611671171494506,
 'recall': 0.9247490843362032,
 'support': 21023}
```

## javascript
### Summary
36 rules, avg.len. 7.4

| | |
|-|-|
|Min support|92|
|Max support|18860|
|Min confidence|0.92514967918396|
|Max confidence|0.9999554753303528|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.995. Support: 110.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 18860.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.999. Support: 833.` |
| 4 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2819.` |
| 5 | `  -1.reserved = {<br>	∧ -5.label in {<newline>}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.980. Support: 171.` |
| 6 | `  -1.reserved = {<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles in {MAP, VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 210.` |
| 7 | `  -1.reserved = {<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ -5.diff_offset ≥ 23<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles in {MAP} and not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 122.` |
| 8 | `  -1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.994. Support: 1680.` |
| 9 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.999. Support: 765.` |
| 10 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE, LITERAL} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 732.` |
| 11 | `  •••start_col ≥ 67<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 136.` |
| 12 | `  -1.diff_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {FILE} and not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.979. Support: 215.` |
| 13 | `  -1.label not in {<space>}<br>	∧ -1.reserved = `<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 359.` |
| 14 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, `, {}<br>	∧ -2.reserved = (<br>	∧ -3.diff_col ≤ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 206.` |
| 15 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, `, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 9888.` |
| 16 | `  •••start_col ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, `, {}<br>	∧ -2.diff_offset ≤ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 1.000. Support: 1805.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2540.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 225.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 2420.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 207.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = `<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 92.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 445.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.949. Support: 2123.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 741.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 466.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles in {NAME}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 324.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 256.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {NAME}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 136.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 130.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ -4.label in {<-tab>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = TryStatement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 100.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;, if}<br>	∧ -1.roles not in {NAME}<br>	∧ -4.label in {<-tab>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, TryStatement}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 195.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;, if}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME}<br>	∧ -4.label not in {<-tab>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 11236.` |
| 35 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 167.` |
| 36 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, ;, if}<br>	∧ -1.roles not in {EXPRESSION, NAME}<br>	∧ -4.label not in {<-tab>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 6392.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.361111111111111, "max_conf": 0.9999554753303528, "max_support": 18860, "min_conf": 0.92514967918396, "min_support": 92, "num_rules": 36}}
```
</details>
