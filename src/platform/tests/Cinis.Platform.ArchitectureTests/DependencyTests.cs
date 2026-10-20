using System.Reflection;
using System.Xml.Linq;

namespace Cinis.Platform.ArchitectureTests;

public sealed class DependencyTests
{
    [Fact]
    public void ProductionAssemblies_ExportCompilerValidatedXmlDocumentation()
    {
        foreach (var name in Allowed.Keys)
        {
            var path = Path.ChangeExtension(Assembly.Load(name).Location, ".xml");
            Assert.True(File.Exists(path), $"Missing production XML documentation: {name}");
            var documentation = XDocument.Load(path);
            Assert.Equal(name, documentation.Root!.Element("assembly")!.Element("name")!.Value);
            Assert.NotEmpty(documentation.Descendants("member"));
        }
    }

    private static readonly Dictionary<string, string[]> Allowed = new()
    {
        ["Cinis.Platform"] = ["Cinis.Access", "Cinis.Access.Infrastructure", "Cinis.Trading", "Cinis.ExecutionClient"],
        ["Cinis.Access"] = [],
        ["Cinis.Trading"] = [],
        ["Cinis.Access.Infrastructure"] = ["Cinis.Access"],
        ["Cinis.ExecutionClient"] = ["Cinis.Trading"]
    };

    private static string PlatformRoot()
    {
        for (var directory = new DirectoryInfo(AppContext.BaseDirectory); directory is not null; directory = directory.Parent)
        {
            var candidate = Path.Combine(directory.FullName, "src", "platform");
            if (File.Exists(Path.Combine(candidate, "Cinis.Platform.slnx"))) return candidate;
        }
        throw new InvalidOperationException("Cannot locate the platform solution.");
    }

    private static void ValidateReference(
        string owner,
        string dependency)
    {
        if (!Allowed[owner].Contains(dependency))
            throw new InvalidOperationException($"Forbidden project dependency: {owner} -> {dependency}");
    }

    [Fact]
    public void ActualProjectGraph_MatchesAllowlist()
    {
        var root = PlatformRoot();
        foreach (var (name, expected) in Allowed)
        {
            var project = XDocument.Load(Path.Combine(root, name, name + ".csproj"));
            var actual = project.Descendants("ProjectReference").Select(x =>
                Path.GetFileNameWithoutExtension(x.Attribute("Include")!.Value.Replace('\\', '/'))).ToArray();
            foreach (var dependency in actual) ValidateReference(name, dependency);
            Assert.Equal(expected.Order(), actual.Order());
        }
        var listed = XDocument.Load(Path.Combine(root, "Cinis.Platform.slnx")).Descendants("Project")
            .Select(x => x.Attribute("Path")!.Value.Replace('\\', '/')).ToArray();
        var projects = Directory.GetFiles(root, "*.csproj", SearchOption.AllDirectories)
            .Select(x => Path.GetRelativePath(root, x).Replace('\\', '/')).ToArray();
        Assert.Equal(projects.Order(), listed.Order());
        var production = projects.Where(x => !x.StartsWith("tests/", StringComparison.Ordinal))
            .Select(Path.GetFileNameWithoutExtension).Order();
        Assert.Equal(Allowed.Keys.Order(), production);
    }

    [Theory]
    [InlineData("Cinis.Access", "Cinis.Access.Infrastructure")]
    [InlineData("Cinis.Trading", "Cinis.ExecutionClient")]
    [InlineData("Cinis.Access.Infrastructure", "Cinis.Trading")]
    public void ForbiddenReference_IsRejected(
        string owner,
        string dependency) =>
        Assert.Throws<InvalidOperationException>(() => ValidateReference(owner, dependency));

    [Theory]
    [InlineData("Cinis.Access")]
    [InlineData("Cinis.Trading")]
    public void CoreAssemblies_DoNotDependOnAdaptersOrWebFrameworks(string name)
    {
        var references = Assembly.Load(name).GetReferencedAssemblies().Select(x => x.Name!).ToArray();
        Assert.DoesNotContain(references, x => x.StartsWith("Microsoft.AspNetCore", StringComparison.Ordinal)
            || x.Contains("EntityFramework", StringComparison.Ordinal) || x.StartsWith("Npgsql", StringComparison.Ordinal)
            || x == "Cinis.ExecutionClient" || x.EndsWith(".Infrastructure", StringComparison.Ordinal));
        var project = XDocument.Load(Path.Combine(PlatformRoot(), name, name + ".csproj"));
        Assert.Empty(project.Descendants("PackageReference"));
        Assert.Empty(project.Descendants("FrameworkReference"));
    }
}
