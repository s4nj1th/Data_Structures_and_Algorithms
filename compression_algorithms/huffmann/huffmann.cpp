#include <bits/stdc++.h>
using namespace std;

#include <functional> // For std::function

template <typename T>
class HuffNode{
public:
    optional<T> val;
    int freq;
    HuffNode<T> *left, *right;

    HuffNode(optional<T> val, int freq){
        this->val = val;
        this->freq = freq;
        left = right = nullptr;
    }
};

template <typename T>
class HuffmannTree{
public:
    HuffNode<T> *root = nullptr;
    function<bool(const T&, const T&)> compare;
     HuffmannTree(function<bool(const T&, const T&)> comparator = [](const T& a, const T& b){return a == b;})
        : compare(comparator) {}

    string huffmann_encode(vector<T>& elements){
        if (elements.empty()) return "";    //Handling empty input
        buildTree(elements);
        
        map<T, string> codes;
        getCodes(root, "", codes);

        string finalEncoding = "";
        for(auto x: elements){
            finalEncoding += codes[x];
        }
        return finalEncoding;
    }

    vector<T> huffmann_decode(string encoding){
        vector<T> decoded;
        HuffNode<T> *curr = root;
        if(curr == nullptr || encoding.empty()) return decoded;     //Handling empty input
        if(curr->val.has_value()){
            if(encoding[0] == '0') decoded.push_back(curr->val.value());    //Handling single element
            return decoded;
        }
        for(auto x: encoding){
            if(x == '0') curr = curr->left;
            else curr = curr->right;
            if(curr->val.has_value()){
                decoded.push_back(curr->val.value());
                curr = root;
            }
        }
        return decoded;
    }
    
    ~HuffmannTree(){
        for_each(nodes.begin(), nodes.end(), [](HuffNode<T> *node){delete node;});
    }

private:
    vector<HuffNode<T>*> nodes;
    
    void getCodes(HuffNode<T> *root, string code, map<T, string> &codes){
        if(root == nullptr) return;
        if(root->val.has_value()) {
            if(code.empty()) code = "0";    //Handling single element
            codes[root->val.value()] = code;
        }
        else{
            getCodes(root->left, code + "0", codes);
            getCodes(root->right, code + "1", codes);
        }
    }

    void buildTree(const vector<T> &elements){
        for_each(nodes.begin(), nodes.end(), [](HuffNode<T> *node){delete node;});
        nodes.clear();
        root = nullptr;

        map<T, int> freq;
        for(const T &x: elements){
            bool found = false;
            for(auto &y: freq){
                if(compare(x, y.first)){
                    found = true;
                    break;
                }
            }
            if(!found) freq.insert({x, 0});
            freq[x]++;
        }
        auto comp = [](HuffNode<T>* a, HuffNode<T>* b) {
            return a->freq > b->freq; // Min-heap based on frequency
        };
        priority_queue<HuffNode<T>*, vector<HuffNode<T>*>, decltype(comp)> pq(comp);

        for(auto x: freq){
            HuffNode<T> *node = new HuffNode<T>(x.first, x.second);
            nodes.push_back(node);
            pq.push(node);
        }

        while (pq.size() > 1){
            HuffNode<T> *left = pq.top();
            pq.pop();
            HuffNode<T> *right = pq.top();
            pq.pop();

            HuffNode<T> *newNode = new HuffNode<T>(nullopt, left->freq + right->freq);
            nodes.push_back(newNode);
            newNode->left = left;
            newNode->right = right;
            pq.push(newNode);
        }

        root = pq.top();
    }
};

struct MyClass {
    int id;
    string name;

    //This is necessary for custom classes/structs as this will be used by the encoder to compare objects
    bool operator<(const MyClass& other) const {
        return id < other.id;
    }
};

int main() {
//Test 1:
    //Huffmann Encoder for characters
    string message = "Hello World!";
    cout<<"Message: "<<message<<'\n';
    vector<char> elements;
    for(auto x: message){
        elements.push_back(x);
    }
    HuffmannTree<char> tree;
    string encoding = tree.huffmann_encode(elements);
    cout << "Encoded: " << encoding << '\n';
    vector<char> decoded = tree.huffmann_decode(encoding);
    cout << "Decoded: ";
    for(auto x: decoded){
        cout << x;
    }
    cout<<'\n';

//Test 2:
    message = "";
    cout<<"Message: "<<message<<'\n';
    elements.clear();
    for(auto x: message){
        elements.push_back(x);
    }
    encoding = tree.huffmann_encode(elements);
    cout << "Encoded: " << encoding << '\n';
    decoded = tree.huffmann_decode(encoding);
    cout << "Decoded: ";
    for(auto x: decoded){
        cout << x;
    }
    cout<<'\n';

//Test 3:
    message = "a";
    cout<<"Message: "<<message<<'\n';
    elements.clear();
    for(auto x: message){
        elements.push_back(x);
    }
    encoding = tree.huffmann_encode(elements);
    cout << "Encoded: " << encoding << '\n';
    decoded = tree.huffmann_decode(encoding);
    cout << "Decoded: ";
    for(auto x: decoded){
        cout << x;
    }
    cout<<'\n';

//Test 4:
    //Huffmann Encoder for custom classes
    vector<MyClass> elements1 = {
        {1, "apple"},
        {2, "banana"},
        {1, "apple"},
        {3, "cherry"},
        {2, "banana"},
        {1, "apple"}
    };
    // Create the Huffman tree using the custom comparator
    HuffmannTree<MyClass> tree1([](const MyClass a, const MyClass b) {
        return a.id == b.id;
    });
    string encoding1 = tree1.huffmann_encode(elements1);
    cout << "Encoded: " << encoding1 << '\n';
    vector<MyClass> decoded1 = tree1.huffmann_decode(encoding1);
    cout << "Decoded: ";
    for (auto x : decoded1) {
        cout << x.id << "(" << x.name << ") ";
    }
    cout << '\n';
    
    return 0;
}