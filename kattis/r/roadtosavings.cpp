#include <vector>
#include <cstdio>
#include <cstring>
#include <iostream>
#include <queue>

using namespace std;

typedef pair<int,int> ii;
typedef vector<ii> vii;
typedef vector<int> vi;


const int INF=1e9;
vector<vii> AL;
vi visited;

vector<vii> parents;
int used=0;

void dfs(int u){
    if(visited[u]==INF){
        visited[u]=0;
        for (auto &[v,w]:parents[u]){
            used+=w;
            dfs(v);
        }
    }
}

int main() {

    int totalW=0;
    int V,E,s,e; scanf("%d %d %d %d",&V,&E,&s,&e);

    

    AL.assign(V+5,vii());
    while(E--){
        int u,v,w; scanf("%d %d %d",&u,&v,&w);
        AL[u].emplace_back(v,w);
        AL[v].emplace_back(u,w);
        totalW+=w;
    }
    //cout<<totalW<<endl;

    vi dist(V+5,INF); dist[s]=0;

    priority_queue<ii,vector<ii>,greater<ii>> pq;pq.push({0,s});


    parents.assign(V+5,vii());



    while (!pq.empty()){
        auto [d,u] =pq.top();pq.pop();

        if (d>dist[u]) continue;

        for (auto &[v,w]:AL[u]){

            if (dist[u]+w<dist[v]){
                while(!parents[v].empty()){
                    parents[v].pop_back();
                }
                //parents[v].assign(0);
                parents[v].emplace_back(u,w);
                dist[v]=dist[u]+w;
                pq.push({dist[v],v});
            } else if(dist[u]+w==dist[v]){
                parents[v].emplace_back(u,w);
            }
            
        }
    }

    visited.assign(V+5,INF);

    dfs(e);
    cout<<totalW-used<<endl;



    // for (int i=0;i<V-1;++i){
    //     bool modified=false;
    //     for(int u=0; u<V;++u){
    //         if (dist[u]!=INF){
    //             for (auto &[v,w]:AL[u]){
    //                 if (dist[u]+w>=dist[v]) continue;

    //                 dist[v]=dist[u]+w;
    //                 modified=true;
    //             }
    //         }
    //     }
    // }

    return 0;
}