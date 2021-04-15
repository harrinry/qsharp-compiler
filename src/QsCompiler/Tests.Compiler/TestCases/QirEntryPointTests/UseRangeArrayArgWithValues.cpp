//------------------------------------------------------------------------------
// This code was generated by a tool.
// <auto-generated />
//------------------------------------------------------------------------------
//
// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#include <fstream>
#include <iostream>
#include <map>
#include <memory>
#include <vector>

#include "CLI11.hpp"

#include "QirContext.hpp"
#include "QirRuntime.hpp"
#include "SimFactory.hpp"

using namespace Microsoft::Quantum;
using namespace std;
    

struct InteropArray
{
    int64_t Size;
    void* Data;

    InteropArray(int64_t size, void* data) :
        Size(size),
        Data(data){}
};

template<typename T>
unique_ptr<InteropArray> CreateInteropArray(vector<T>& v)
{
    unique_ptr<InteropArray> array(new InteropArray(v.size(), v.data()));
    return array;
}

template<typename S, typename D>
void TranslateVector(vector<S>& sourceVector, vector<D>& destinationVector, function<D(S&)> translationFunction)
{
    destinationVector.resize(sourceVector.size());
    transform(sourceVector.begin(), sourceVector.end(), destinationVector.begin(), translationFunction);
}

using RangeTuple = tuple<int64_t, int64_t, int64_t>;
struct InteropRange
{
    int64_t Start;
    int64_t Step;
    int64_t End;

    InteropRange() :
        Start(0),
        Step(0),
        End(0){}

    InteropRange(RangeTuple rangeTuple) :
        Start(get<0>(rangeTuple)),
        Step(get<1>(rangeTuple)),
        End(get<2>(rangeTuple)){}
};

unique_ptr<InteropRange> CreateInteropRange(RangeTuple rangeTuple)
{
    unique_ptr<InteropRange> range(new InteropRange(rangeTuple));
    return range;
}

InteropRange* TranslateRangeTupleToInteropRangePointer(RangeTuple& rangeTuple)
{
    InteropRange* range = new InteropRange(rangeTuple);
    return range;
}

template<typename T>
void FreePointerVector(vector<T*>& v)
{
    for (auto p : v)
    {
        delete p;
    }
}

// This is the function corresponding to the QIR entry-point.
extern "C" void UseRangeArrayArgWithValues( // NOLINT
    InteropArray* RangeArrayArgInteropValue
);


int main(int argc, char* argv[])
{
    CLI::App app("QIR Standalone Entry Point Inputs Reference");

    // Initialize simulator.
    unique_ptr<IRuntimeDriver> sim = CreateFullstateSimulator();
    QirContextScope qirctx(sim.get(), false /*trackAllocatedObjects*/);

    // Add the --simulation-output options.
    // N.B. This option should be present in all standalone drivers.
    string simulationOutputFile;
    CLI::Option* simulationOutputFileOpt = app.add_option(
        "--simulation-output", simulationOutputFile,
        "File where the output produced during the simulation is written");

    vector<RangeTuple> RangeArrayArgCliValue;
    app.add_option("--RangeArrayArg", RangeArrayArgCliValue, "A Range array value for the RangeArrayArg argument")->required();

    // With all the options added, parse arguments from the command line.
    CLI11_PARSE(app, argc, argv);

    vector<InteropRange*> RangeArrayArgIntermediateValue;
    TranslateVector<RangeTuple, InteropRange*>(RangeArrayArgCliValue, RangeArrayArgIntermediateValue, TranslateRangeTupleToInteropRangePointer);
    unique_ptr<InteropArray> RangeArrayArgInteropValue = CreateInteropArray(RangeArrayArgIntermediateValue);

    // Redirect the simulator output from std::cout if the --simulation-output option is present.
    ostream* simulatorOutputStream = &cout;
    ofstream simulationOutputFileStream;
    if (!simulationOutputFileOpt->empty())
    {
        simulationOutputFileStream.open(simulationOutputFile);
        SetOutputStream(simulationOutputFileStream);
        simulatorOutputStream = &simulationOutputFileStream;
    }

    // Run simulation and write the output of the operation to the corresponding stream.
    UseRangeArrayArgWithValues(
        RangeArrayArgInteropValue.get()
    );


    simulatorOutputStream->flush();
    if (simulationOutputFileStream.is_open())
    {
        simulationOutputFileStream.close();
    }
}
